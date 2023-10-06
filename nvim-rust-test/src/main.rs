extern crate rand;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=100);
    println!("Random number: {}", random_number);
}
