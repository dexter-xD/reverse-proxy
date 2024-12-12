use async_trait::async_trait;
use pingora::prelude::*;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_load_balancing::selection::RoundRobin;
use pingora_load_balancing::LoadBalancer;
use pingora_proxy::{ProxyHttp, Session};
use pingora::http::ResponseHeader;
use std::sync::Arc;

use crate::rate_limiter::{RATE_LIMITER, MAX_REQ_PER_SEC};

pub struct LB(pub Arc<LoadBalancer<RoundRobin>>);

impl LB {
    pub fn get_request_appid(&self, session: &mut Session) -> Option<String> {
        match session
            .req_header()
            .headers
            .get("appid")
            .map(|v| v.to_str())
        {
            None => None,
            Some(v) => match v {
                Ok(v) => Some(v.to_string()),
                Err(_) => None,
            },
        }
    }
}

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut Self::CTX) -> Result<Box<HttpPeer>> {
        let upstream = self.0.select(b"", 256).unwrap();
        println!("upstream peer is: {upstream:?}");
        
        let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request.insert_header("Host", "one.one.one.one").unwrap();
        Ok(())
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        let appid = match self.get_request_appid(session) {
            None => return Ok(false),
            Some(addr) => addr,
        };

        let curr_window_request = RATE_LIMITER.observe(&appid, 1);
        if curr_window_request > MAX_REQ_PER_SEC {
            let mut header = ResponseHeader::build(429, None).unwrap();
            header.insert_header("X-Rate-Limit-Limit", MAX_REQ_PER_SEC.to_string()).unwrap();
            header.insert_header("X-Rate-Limit-Remaining", "0").unwrap();
            header.insert_header("X-Rate-Limit-Reset", "1").unwrap();
            session.set_keepalive(None);
            session.write_response_header(Box::new(header), true).await?;
            return Ok(true);        
        }
        Ok(false)
    }
}
