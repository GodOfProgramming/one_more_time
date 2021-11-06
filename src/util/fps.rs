use std::{
  thread,
  time::{Duration, Instant},
};

const NANOS_IN_SECS: u64 = 1_000_000_000;

pub struct FpsManager {
  base_sleep_time: Duration,
  start: Instant,
  target: u64,
}

impl FpsManager {
  pub fn new(target: u64) -> Self {
    let base_sleep_time = Duration::from_nanos(NANOS_IN_SECS / target);

    Self {
      base_sleep_time,
      start: Instant::now(),
      target,
    }
  }

  pub fn begin(&mut self) {
    self.start = Instant::now();
  }

  pub fn end(&mut self) {
    thread::sleep(
      self
        .base_sleep_time
        .saturating_sub(Instant::now() - self.start),
    );
  }

  pub fn target(&self) -> u64 {
    self.target
  }
}
