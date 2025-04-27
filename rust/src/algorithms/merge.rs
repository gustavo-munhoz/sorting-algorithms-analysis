use crate::metrics::Metrics;
use super::base::SortAlgorithm;

pub struct MergeSort;

impl SortAlgorithm for MergeSort {
    fn name(&self) -> &'static str {
        "MergeSort"
    }

    fn sort(&self, data: &mut [i32], m: &mut Metrics) {
        if data.len() <= 1 {
            return;
        }

        let middle = data.len() / 2;
        let (left,right) = data.split_at_mut(middle);
        self.sort(left, m);
        self.sort(right, m);

        let sorted = self.merge(left, right, m);

        data.copy_from_slice(&sorted);
    }
}

impl MergeSort {
    fn merge(&self, data1:  &[i32], data2: &[i32], m: &mut Metrics) -> Vec<i32> {
        let mut sorted = Vec::with_capacity(data1.len() + data2.len());

        let mut index1 = 0;
        let mut index2 = 0;

        while index1 < data1.len() && index2 < data2.len() {
            m.comparisons += 1;
            if data1[index1] <= data2[index2] {
                sorted.push(data1[index1]);
                index1 += 1;
            }else {
                sorted.push(data2[index2]);
                index2 += 1;
            }
        }

        if index1 < data1.len() {
            sorted.extend_from_slice(&data1[index1..]);
        } else {
            sorted.extend_from_slice(&data2[index2..]);
        }
        sorted
    }
}