use super::base::SortAlgorithm;

pub struct QuickSort;

impl SortAlgorithm for QuickSort {
    fn name(&self) -> &'static str {
        "QuickSort"
    }

    fn sort(&self, data: &mut [i32]) {
        todo!()
    }
}