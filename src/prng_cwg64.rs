pub struct Cwg64 {
    x: u64,
    a: u64,
    weyl: u64,
    s: u64,
}

impl Cwg64 {
    // Library for multithreaded RNG seeding
    fn splitmix64(seed: &mut u64) -> u64 {
        *seed = seed.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = *seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    fn splitmix63(seed: &mut u64) -> u64 {
        *seed = seed.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = *seed & 0x7FFFFFFFFFFFFFFF;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9) & 0x7FFFFFFFFFFFFFFF;
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB) & 0x7FFFFFFFFFFFFFFF;
        z ^ (z >> 31)
    }

    fn new(seed: &mut u64) -> Cwg64 {
        let x = Self::splitmix64(seed);
        let s = (Self::splitmix63(seed) << 1) | 1;

        Cwg64 {
            x,
            a: 0,
            weyl: 0,
            s,
        }
    }
}

impl Iterator for Cwg64 {
    type Item = u64;

    // Collatz-Weyl Generator - CWG64
    fn next(&mut self) -> Option<Self::Item> {
        self.weyl = self.weyl.wrapping_add(self.s);
        self.a = self.a.wrapping_add(self.x);
        self.x = (self.x >> 1).wrapping_mul(self.a | 1) ^ self.weyl;
        Some(self.a >> 48 ^ self.x)
    }
}

pub fn initialize(seed: &mut u64) -> Cwg64 {
    Cwg64::new(seed)
}
