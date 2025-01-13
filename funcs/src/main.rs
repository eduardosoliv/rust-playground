mod closures;
mod closures_input_params;
mod closures_output_params;
mod functions;
mod functions_params;
mod methods;

use closures::{
    closures_capturing, closures_capturing_copy, closures_capturing_move, closures_example,
};
use closures_input_params::closure_input;
use closures_output_params::closures_output_param;
use functions::{fizzbuzz_to, is_divisible_by};
use functions_params::function_as_parameter;
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
    println!("---");
    closures_capturing();
    println!("---");
    closures_capturing_move();
    closures_capturing_copy();
    println!("---");
    closure_input();

    println!("------");

    function_as_parameter();

    println!("------");

    closures_output_param();
}
