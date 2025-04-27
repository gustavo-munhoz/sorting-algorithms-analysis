#[derive(Debug, Default, serde::Serialize)]
pub struct Metrics {
    pub comparisons: u64,
    pub swaps: u64,
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }
}