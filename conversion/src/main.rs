mod frominto;
mod parsestring;
mod tostring;
mod tryfrominto;

use frominto::{from, into};
use parsestring::{parse_from_string, parse_string};
use tostring::{circle, name};
use tryfrominto::{try_from, try_into};

fn main() {
    from();
    into();
    println!("------");
    try_from();
    try_into();
    println!("------");
    circle();
    name();
    println!("------");
    parse_string();
    parse_from_string("    3 ");
    parse_from_string(" invalid ");
}
