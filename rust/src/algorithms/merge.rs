use super::base::SortAlgorithm;

pub struct MergeSort;

impl SortAlgorithm for MergeSort {
    fn name(&self) -> &'static str {
        "MergeSort"
    }

    fn sort(&self, data: &mut [i32]) {
        todo!()
    }
}