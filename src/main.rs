pub mod get_seed;
pub mod prng_cwg64;
pub mod shuffle;

use chrono::TimeZone;
use chrono_tz::Asia::Seoul;
use clap::{App, Arg};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("factos")
        .version("0.1.0")
        .author("remy2019 <remy2019@gmail.com>")
        .about("Fair and clear time-originated sequence generator")
        .arg(
            Arg::with_name("time")
                .value_name("time")
                .short("t")
                .long("time")
                .help("Input datetime in KST as YYYY-MM-DD-HH-MM")
                .required(true),
        )
        .arg(
            Arg::with_name("length")
                .value_name("length")
                .short("l")
                .long("length")
                .help("Input length of sequence")
                .required(true),
        )
        .arg(
            Arg::with_name("starting index")
                .value_name("start")
                .short("s")
                .long("start")
                .help("Set the starting number of sequence::default = 0")
                .default_value("0"),
        )
        .arg(
            Arg::with_name("phrase")
                .value_name("phrase")
                .short("p")
                .long("phrase")
                .help("Optional phrase to make sequence unique")
                .required(false),
        )
        .get_matches();
    let length = matches.value_of("length").unwrap().parse::<u32>().unwrap();
    let start = matches
        .value_of("starting index")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut seq: Vec<u32> = (0..length).map(|x| x + start).collect();

    let time = matches
        .value_of("time")
        .unwrap()
        .split("-")
        .map(|t| t.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    if time.len() != 5 {
        panic!();
    }
    let t = Seoul
        .with_ymd_and_hms(time[0] as i32, time[1], time[2], time[3], time[4], 0)
        .single()
        .unwrap()
        .to_utc()
        .timestamp_millis();
    println!("{:?}", t);

    let mut seed = get_seed::get_seed();
    shuffle::shuffle(&mut seed, length, &mut seq);

    println!("{:?}", seq);

    // Test getter
    let url = format!("https://beacon.nist.gov/beacon/2.0/pulse/time/{}", t);
    let body: String = ureq::get(url).call()?.body_mut().read_to_string()?;

    let regex = Regex::new(r#"hour\D+:\D+"(?<seed>\w*)"#).unwrap();
    match regex.captures(body.as_str()) {
        Some(res) => println!("{:?}", &res["seed"]),
        None => println!("Wrong datetime"),
    };

    Ok(())
}
