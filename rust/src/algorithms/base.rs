use crate::metrics::Metrics;

/// Common trait for sorting algorithms.
pub trait SortAlgorithm: Send + Sync {

    /// Returns the name of the algorithm.
    fn name(&self) -> &'static str;

    /// Sorts the data *in-place*.
    fn sort(&self, data: &mut [i32], m: &mut Metrics);
}