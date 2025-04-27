use super::base::SortAlgorithm;

pub struct SelectionSort;

impl SortAlgorithm for SelectionSort {
    fn name(&self) -> &'static str {
        "SelectionSort"
    }

    fn sort(&self, data: &mut [i32]) {
        // todo!()
    }
}