use regex::Regex;
use std::str::FromStr;

pub fn get_seed(tmili: i64) -> u64 {
    let url = format!("https://beacon.nist.gov/beacon/2.0/pulse/time/{}", tmili);
    let body: String = ureq::get(url)
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();

    let regex = Regex::new(r#"hour\D+:\D+"(?<seed>\w*)"#).unwrap();
    let res = regex.captures(&body).unwrap();
    let raw_seed = &res["seed"];
    let raw_seed_trim = String::from_str(&raw_seed[0..16]).unwrap();
    u64::from_str_radix(&raw_seed_trim, 16).unwrap()
}
