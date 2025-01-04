mod frominto;
mod tostring;
mod tryfrominto;

use frominto::{from, into};
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
}
