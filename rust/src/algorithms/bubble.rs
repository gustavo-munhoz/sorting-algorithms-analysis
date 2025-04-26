use super::base::SortAlgorithm;

pub struct BubbleSort;

impl SortAlgorithm for BubbleSort {
    fn name(&self) -> &'static str {
        "BubbleSort"
    }

    fn sort(&self, data: &mut [i32]) {
        todo!()
    }
}