use super::base::SortAlgorithm;

pub struct InsertionSort;

impl SortAlgorithm for InsertionSort {
    fn name(&self) -> &'static str {
        "InsertionSort"
    }

    fn sort(&self, data: &mut [i32]) {
        for i in 1..data.len() {
            let key = data[i];
            let mut j = i;
            while j > 0 && data[j - 1] > key {
                data[j] = data[j - 1];
                j -= 1;
            }
            data[j] = key;
    }
    }
}