use std::mem::swap;
use super::base::SortAlgorithm;

pub struct QuickSort;

impl SortAlgorithm for QuickSort {
    fn name(&self) -> &'static str {
        "QuickSort"
    }

    fn sort(&self, data: &mut [i32]) {
        self.quick_sort(data, 0, data.len() - 1);
    }
}

impl QuickSort {
    fn quick_sort(&self, data: &mut [i32], low: usize, high: usize) {
        if low < high {
            let pivot_location = self.partition(data, low, high);
            self.quick_sort(data, low, pivot_location);
            self.quick_sort(data, pivot_location + 1, high);
        }
    }

    fn partition(&self, data: &mut [i32], low: usize, high: usize) -> usize {
        let pivot = data[low];
        let mut left_wall = low;

        for i in (low + 1)..=high {
            if data[i] < pivot {
                left_wall += 1;
                data.swap(i, left_wall);
            }
        }

        data.swap(low, left_wall);

        left_wall
    }
}