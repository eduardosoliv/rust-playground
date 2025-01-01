mod compound;
use compound::{
    arrays::{access_array, arrays, slices},
    main::{array, tuple},
    tuples::{destruct_tuple, matrix, pair, tuple_extract, tuple_of_tuples, tuple_vs_literal},
};

mod scalar;
use scalar::{
    literals_ops::{bitwise_ops, boolean_ops, literals},
    main::scalars,
};

fn main() {
    scalars();
    array();
    tuple();
    println!("------------------");
    literals();
    boolean_ops();
    bitwise_ops();
    println!("------------------");
    tuple_extract();
    tuple_of_tuples();
    pair();
    tuple_vs_literal();
    destruct_tuple();
    println!("------------------");
    matrix();
    println!("------------------");
    arrays();
    println!("---");
    slices();
    println!("---");
    access_array();
}
