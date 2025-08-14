use crate::prng_cwg64;

// Fisher-Yates shuffle
pub fn shuffle(seed: &mut u64, length: u32, seq: &mut [u32]) {
    let mut prng = prng_cwg64::initialize(seed);

    for i in (1..length).rev() {
        let j = u32::try_from(prng.next().unwrap() % (u64::from(i) + 1)).unwrap();
        seq.swap(i as usize, j as usize);
    }
}
