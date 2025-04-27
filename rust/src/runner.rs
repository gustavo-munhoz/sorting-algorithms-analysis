use std::time::Instant;

use crate::{
    algorithms::base::SortAlgorithm,
    dataset,
    metrics::Metrics,
};

#[derive(serde::Serialize)]
pub struct Record {
    pub algorithm: String,
    pub n: usize,
    pub elapsed_ns: u128,
}

pub fn run(
    algorithms: &[Box<dyn SortAlgorithm>],
    sizes: &[usize],
) -> Vec<Record> {
    let mut results: Vec<Record> = Vec::new();

    for &n in sizes {
        let base_data = dataset::random_vec(n);
        for algo in algorithms {
            let mut data = base_data.clone();
            let mut m = Metrics::new();

            let start = Instant::now();
            algo.sort(&mut data);

            m.elapsed = start.elapsed();

            results.push(Record {
                algorithm: algo.name().into(),
                n,
                elapsed_ns: m.elapsed.as_nanos(),
            });
        }
    }

    results
}

pub fn run_one(algorithm: &dyn SortAlgorithm, n: usize) -> std::io::Result<Record> {
    use std::time::Instant;
    use crate::dataset::random_vec;

    let mut data = random_vec(n);
    let start = Instant::now();
    algorithm.sort(&mut data);
    Ok(Record {
        algorithm: algorithm.name().into(),
        n,
        elapsed_ns: start.elapsed().as_nanos(),
    })
}
