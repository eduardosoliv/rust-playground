mod compound;
use crate::compound::main::array;
use crate::compound::main::tuple;

mod scalar;
use scalar::literals_ops::bitwise_ops;
use scalar::literals_ops::boolean_ops;
use scalar::literals_ops::literals;
use scalar::main::scalars;

fn main() {
    scalars();
    array();
    tuple();
    println!("------------------");
    literals();
    boolean_ops();
    bitwise_ops();
}
