mod load_balancer;
mod rate_limiter;

use pingora::prelude::*;
use pingora_core::services::background::background_service;
use std::{sync::Arc, time::Duration};
use pingora_load_balancing::{selection::RoundRobin, LoadBalancer};

fn main() {
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    let mut upstreams =
        LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443", "127.0.0.1:343"]).unwrap();

    let hc = TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(Duration::from_secs(1));

    let background = background_service("health check", upstreams);
    let upstreams = background.task();

    let mut lb = http_proxy_service(&my_server.configuration, load_balancer::LB(Arc::clone(&upstreams)));
    lb.add_tcp("0.0.0.0:6188");
    println!("Listening on: 0.0.0.0:6188");

    my_server.add_service(background);
    my_server.add_service(lb);
    my_server.run_forever();
}
