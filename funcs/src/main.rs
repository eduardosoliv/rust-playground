mod closures;
mod functions;
mod methods;

use closures::closures_example;
use functions::{fizzbuzz_to, is_divisible_by};
use methods::methods_example;
fn main() {
    println!("is divisible by");
    println!("20 / 0 = {}", is_divisible_by(20, 0));
    println!("20 / 5 = {}", is_divisible_by(20, 5));
    println!("20 / 3 = {}", is_divisible_by(20, 3));

    fizzbuzz_to(20);

    println!("------");

    methods_example();

    println!("------");

    closures_example();

    println!("------");
}
