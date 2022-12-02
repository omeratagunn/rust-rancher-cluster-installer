use std::time::{Instant};


pub(crate) fn start_time() -> Instant{
    Instant::now()
}
pub(crate) fn finish_time(start_time: Instant) -> u128{
    start_time.elapsed().as_millis()
}
