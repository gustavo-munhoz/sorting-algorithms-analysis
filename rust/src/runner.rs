use crate::{
    algorithms::base::SortAlgorithm,
    metrics::Metrics,
};

#[derive(serde::Serialize)]
pub struct Record {
    pub algorithm: String,
    pub n: usize,
    pub elapsed_ns: u128,
    pub comparisons: u64,
    pub swaps: u64,
}

pub fn run_one(algorithm: &dyn SortAlgorithm, n: usize) -> std::io::Result<Record> {
    use std::time::Instant;
    use crate::dataset::random_vec;

    let mut m = Metrics::new();
    let mut data = random_vec(n);
    let start = Instant::now();
    algorithm.sort(&mut data, &mut m);
    Ok(Record {
        algorithm: algorithm.name().into(),
        n,
        elapsed_ns: start.elapsed().as_nanos(),
        comparisons: m.comparisons,
        swaps: m.swaps,
    })
}
