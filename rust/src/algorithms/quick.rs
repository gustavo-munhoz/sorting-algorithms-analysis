use crate::metrics::Metrics;
use super::base::SortAlgorithm;

pub struct QuickSort;

impl SortAlgorithm for QuickSort {
    fn name(&self) -> &'static str {
        "QuickSort"
    }

    fn sort(&self, data: &mut [i32], m: &mut Metrics) {
        if data.len() < 2 {
            return;
        }
        self.quick_sort(data, 0, data.len() - 1, m);
    }
}

impl QuickSort {
    fn quick_sort(&self, data: &mut [i32], low: usize, high: usize, m: &mut Metrics) {
        if low < high {
            let pivot_location = self.partition(data, low, high, m);
            if pivot_location > 0 {
                self.quick_sort(data, low, pivot_location, m);
            }
            self.quick_sort(data, pivot_location + 1, high, m);
        }
    }

    fn partition(&self, data: &mut [i32], low: usize, high: usize, m: &mut Metrics) -> usize {
        let pivot = data[low];
        let mut left_wall = low;

        for i in (low + 1)..=high {
            m.comparisons += 1;
            if data[i] < pivot {
                left_wall += 1;
                data.swap(i, left_wall);
                m.swaps += 1;
            }
        }

        data.swap(low, left_wall);
        m.swaps += 1;

        left_wall
    }
}