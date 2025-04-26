use std::time::Duration;

#[derive(Debug, Default, serde::Serialize)]
pub struct Metrics {
    pub comparisons: u64,
    pub swaps: u64,
    pub elapsed: Duration,
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }
}