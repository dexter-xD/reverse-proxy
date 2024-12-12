use once_cell::sync::Lazy;
use pingora_limits::rate::Rate;
use std::time::Duration;

pub static RATE_LIMITER: Lazy<Rate> = Lazy::new(|| Rate::new(Duration::from_secs(1)));

pub static MAX_REQ_PER_SEC: isize = 1;
