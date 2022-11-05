extern crate rand;
extern crate rand_chacha;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

fn main() {
    let mut memoized_rng = |seed: u64| {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        rng.gen::<u64>()
    };
    println!("{}", memoized_rng(0));
    println!("{}", memoized_rng(0));
    println!("{}", memoized_rng(1));
    println!("{}", memoized_rng(1));
}
