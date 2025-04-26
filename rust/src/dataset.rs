use rand::{Rng, seq::index::sample};

pub fn random_vec(n: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..n).map(|_| rng.random_range(0..n as i32)).collect()
}

pub fn nearly_sorted(n: usize, swaps: usize) -> Vec<i32> {
    let mut v: Vec<i32> = (0..n as i32).collect();
    let mut rng = rand::rng();

    for pair in sample(&mut rng, n, swaps * 2).into_vec().chunks_exact(2) {
        v.swap(pair[0], pair[1]);
    }

    v
}