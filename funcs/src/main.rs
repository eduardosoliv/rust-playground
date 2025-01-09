mod functions;

use functions::{fizzbuzz_to, is_divisible_by};
fn main() {
    println!("is divisible by");
    println!("20 / 0 = {}", is_divisible_by(20, 0));
    println!("20 / 5 = {}", is_divisible_by(20, 5));
    println!("20 / 3 = {}", is_divisible_by(20, 3));

    fizzbuzz_to(20);
}
