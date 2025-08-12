use std::str::FromStr;

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

// Collatz-Weyl Generator - CWG64
fn cwg64(x: &mut u64, a: &mut u64, weyl: &mut u64, s: u64) -> u64 {
    *weyl = weyl.wrapping_add(s);
    *a = a.wrapping_add(*x);
    *x = (*x >> 1).wrapping_mul(*a | 1) ^ *weyl;
    *a >> 48 ^ *x
}

fn main() {
    // Arguments
    let _length = 50;

    let raw_seed = "57C7105DF836A0532C3C7A15D6C07BC6A6D2869A8BBC770B099E056BD468179407E5B1551F5653F98E51CCFB07326C7245EF5D8BFE7D8B686DB662A7D481DEDF";
    let raw_seed_trim = String::from_str(&raw_seed[0..16]).unwrap();
    let mut seed = u64::from_str_radix(&raw_seed_trim, 16).unwrap();

    // Initialization
    let mut a = 0;
    let mut weyl = 0;
    let mut x = splitmix64(&mut seed);
    let s = (splitmix63(&mut seed) << 1) | 1;

    loop {
        let stream = cwg64(&mut x, &mut a, &mut weyl, s);
        let len_stream = stream.checked_ilog10().unwrap_or(0) + 1;
        if len_stream == 11 {
            println!("{}", len_stream);
        }
    }
}
