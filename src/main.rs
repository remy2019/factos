pub mod prng_cwg64;

use std::str::FromStr;

fn main() {
    let raw_seed = "3AC1105DF836A0532C3C7A15D6C07BC6A6D2869A8BBC770B099E056BD468179407E5B1551F5653F98E51CCFB07326C7245EF5D8BFE7D8B686DB662A7D481DEDF";
    let raw_seed_trim = String::from_str(&raw_seed[0..16]).unwrap();
    let mut seed = u64::from_str_radix(&raw_seed_trim, 16).unwrap();

    let mut prng = prng_cwg64::initialize(&mut seed);

    let length = 31;
    let mut seq: Vec<u32> = (0..length).collect();
    println!("{:?}", seq);

    // Fisher-Yates shuffle
    for i in (1..length).rev() {
        let j = u32::try_from(prng.next().unwrap() % (u64::from(i) + 1)).unwrap();
        seq.swap(i as usize, j as usize);
        println!("{:?}, {:?}, {:?}", i, j, seq);
    }
}
