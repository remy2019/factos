use crate::prng_cwg64;
use std::hash::{DefaultHasher, Hasher};

// Fisher-Yates shuffle
pub fn shuffle(seed: &mut u64, phrase: Option<&str>, length: u32, seq: &mut [u32]) {
    let mut seed = match phrase {
        Some(p) => {
            let mut hasher = DefaultHasher::new();
            hasher.write(p.as_bytes());
            *seed ^ hasher.finish()
        }
        None => *seed,
    };
    let mut prng = prng_cwg64::initialize(&mut seed);

    for i in (1..length).rev() {
        let j = u32::try_from(prng.next().unwrap() % (u64::from(i) + 1)).unwrap();
        seq.swap(i as usize, j as usize);
    }
}
