pub mod get_seed;
pub mod prng_cwg64;
pub mod shuffle;

fn main() {
    let length = 31;
    let mut seq: Vec<u32> = (0..length).collect();

    let mut seed = get_seed::get_seed();
    shuffle::shuffle(&mut seed, length, &mut seq);

    println!("{:?}", seq);
}
