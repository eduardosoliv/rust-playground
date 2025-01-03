mod casting;
use casting::{float_to_int, int_to_char, overflow};

mod literals;
use literals::literals;

mod inference;
use inference::inference;

mod aliasing;
use aliasing::aliasing;

fn main() {
    let decimal = 65.4321_f32;

    let integer = float_to_int(decimal);

    println!(
        "Casting: {} -> {} -> {}",
        decimal,
        integer,
        int_to_char(integer as u8)
    );

    overflow();

    println!("---");

    literals();

    println!("---");

    inference();

    println!("---");

    aliasing();
}
