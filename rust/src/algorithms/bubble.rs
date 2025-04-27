use crate::metrics::Metrics;
use super::base::SortAlgorithm;

pub struct BubbleSort;

impl SortAlgorithm for BubbleSort {
    fn name(&self) -> &'static str {
        "BubbleSort"
    }

    fn sort(&self, data: &mut [i32], m: &mut Metrics) {
        for i in 1..data.len() {
            for j in 0..data.len() - i {
                m.comparisons += 1;
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                    m.swaps += 1;
                }
            }
        }
    }
}