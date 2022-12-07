use std::time::{Instant};


pub(crate) fn start_time() -> Instant{
    Instant::now()
}
pub(crate) fn finish_time(start_time: Instant) -> u32{
    start_time.elapsed().as_secs() as u32
}
