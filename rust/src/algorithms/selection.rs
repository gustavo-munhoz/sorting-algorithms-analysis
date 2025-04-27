use super::base::SortAlgorithm;

pub struct SelectionSort;

impl SortAlgorithm for SelectionSort {
    fn name(&self) -> &'static str {
        "SelectionSort"
    }

    fn sort(&self, data: &mut [i32]) {
        for i in 0..data.len() {
            let mut min_index = i;
            for j in i+1..data.len() {
                if data[j] < data[min_index] {
                    min_index = j;
                }
            }
            data.swap(i, min_index);
        }
    }
}