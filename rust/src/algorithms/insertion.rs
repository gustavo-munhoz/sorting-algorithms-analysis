use super::base::SortAlgorithm;

pub struct InsertionSort;

impl SortAlgorithm for InsertionSort {
    fn name(&self) -> &'static str {
        "InsertionSort"
    }

    fn sort(&self, data: &mut [i32]) {
        // todo!()
    }
}