mod algorithms;
mod report;
mod runner;
mod dataset;
mod metrics;

use crate::algorithms::{
    base::SortAlgorithm,
    bubble::BubbleSort,
    insertion::InsertionSort,
    merge::MergeSort,
    quick::QuickSort,
    selection::SelectionSort,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let algorithms: Vec<Box<dyn SortAlgorithm>> = vec![
        Box::new(InsertionSort),
        Box::new(SelectionSort),
        Box::new(BubbleSort),
        Box::new(MergeSort),
        Box::new(QuickSort),
    ];

    let sizes = [1_000, 100_000, 1_000_000];

    let results = runner::run(&algorithms, &sizes);
    report::to_json("rust_results.json", &results)?;

    println!("Saved { } results to rust_results.json.", results.len());
    Ok(())
}
