// A Rust program is (mostly) made up of a series of statements:
fn main() {
    println!("Expressions");
    // statement
    // statement
    // statement
    expressions();
    blocks();
}

// There are a few kinds of statements in Rust.
// The most common two are declaring a variable binding, and using a ; with an expression:
#[allow(path_statements)]
#[allow(unused_must_use)]
fn expressions() {
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;
}

// Blocks are expressions too, so they can be used as values in assignments.
// The last expression in the block will be assigned to the place expression such as a local variable.
// However, if the last expression of the block ends with a semicolon, the return value will be ().
#[allow(unused_must_use)]
fn blocks() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    let z_working = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
    println!("z (working) is {:?}", z_working);
}
