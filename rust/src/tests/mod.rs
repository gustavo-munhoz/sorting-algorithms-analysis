use crate::algorithms::{
    base::SortAlgorithm,
    bubble::BubbleSort,
    insertion::InsertionSort,
    merge::MergeSort,
    quick::QuickSort,
    selection::SelectionSort,
};
use crate::metrics::Metrics;

use std::sync::Mutex;
use once_cell::sync::Lazy;

static RESULTS: Lazy<Mutex<Vec<(String, bool)>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn run_sort_tests(algorithm: &dyn SortAlgorithm) {
    let test_cases = vec![
        vec![],
        vec![1],
        vec![2, 1],
        vec![1, 2, 3, 4, 5],
        vec![5, 4, 3, 2, 1],
        vec![3, 1, 2, 3, 1, 2],
        (100..0).rev().collect::<Vec<_>>(),
    ];

    let mut passed = true;

    for original in test_cases {
        let mut data = original.clone();
        let mut metrics = Metrics::default();

        println!("Testing {:<15} input: {:?}", algorithm.name(), original);

        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            algorithm.sort(&mut data, &mut metrics);
        })).is_err() {
            passed = false;
            break;
        }

        let mut expected = original.clone();
        expected.sort();

        if data != expected {
            passed = false;
            break;
        }
    }

    RESULTS.lock().unwrap().push((algorithm.name().to_string(), passed));

    if passed {
        println!("✅ {:<15} PASSED", algorithm.name());
    } else {
        println!("❌ {:<15} FAILED", algorithm.name());
        panic!("Algorithm {} failed some test cases!", algorithm.name());
    }
}

#[test]
fn test_bubble_sort() {
    let algo = BubbleSort;
    run_sort_tests(&algo);
}

#[test]
fn test_insertion_sort() {
    let algo = InsertionSort;
    run_sort_tests(&algo);
}

#[test]
fn test_merge_sort() {
    let algo = MergeSort;
    run_sort_tests(&algo);
}

#[test]
fn test_quick_sort() {
    let algo = QuickSort;
    run_sort_tests(&algo);
}

#[test]
fn test_selection_sort() {
    let algo = SelectionSort;
    run_sort_tests(&algo);
}

#[test]
fn print_summary() {
    println!("\n--- Summary ---");
    for (alg, passed) in RESULTS.lock().unwrap().iter() {
        if *passed {
            println!("✅ {} PASSED", alg);
        } else {
            println!("❌ {} FAILED", alg);
        }
    }
    println!("----------------\n");
}