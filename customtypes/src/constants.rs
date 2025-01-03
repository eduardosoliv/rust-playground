// Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:
// const: An unchangeable value (the common case).
// static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified.
// Accessing or modifying a mutable static variable is unsafe.

// Globals are declared outside all other scopes.
pub static LANGUAGE: &str = "Rust";
pub const LANGUAGE2: &str = "Python"; // Fixed: Changed to &'static str
pub const THRESHOLD: i32 = 10;

pub fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}
