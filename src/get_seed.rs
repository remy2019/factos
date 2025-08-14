use std::str::FromStr;

pub fn get_seed() -> u64 {
    let raw_seed = "3AC1105DF836A0532C3C7A15D6C07BC6A6D2869A8BBC770B099E056BD468179407E5B1551F5653F98E51CCFB07326C7245EF5D8BFE7D8B686DB662A7D481DEDF";
    let raw_seed_trim = String::from_str(&raw_seed[0..16]).unwrap();
    u64::from_str_radix(&raw_seed_trim, 16).unwrap()
}
