mod frominto;
mod tryfrominto;

use frominto::{from, into};
use tryfrominto::{try_from, try_into};

fn main() {
    from();
    into();
    println!("------");
    try_from();
    try_into();
}
