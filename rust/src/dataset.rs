use rand::{Rng, seq::index::sample};

pub fn random_vec(n: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..n).map(|_| rng.random_range(0..n as i32)).collect()
}