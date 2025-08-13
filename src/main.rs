pub mod prng_cwg64;

use std::str::FromStr;

fn main() {
    let raw_seed = "57C7105DF836A0532C3C7A15D6C07BC6A6D2869A8BBC770B099E056BD468179407E5B1551F5653F98E51CCFB07326C7245EF5D8BFE7D8B686DB662A7D481DEDF";
    let raw_seed_trim = String::from_str(&raw_seed[0..16]).unwrap();
    let mut seed = u64::from_str_radix(&raw_seed_trim, 16).unwrap();

    let mut prng = prng_cwg64::initialize(&mut seed);

    for _ in 1..10 {
        println!("{:?}", prng.next());
    }
}
