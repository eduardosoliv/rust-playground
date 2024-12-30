fn main() {
    primitives_scalar();
    primitives_compound();
    println!("---------");
    literals_operators();
}

fn primitives_scalar() {
    // Scalar types:
    //  Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    //  Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    //  Floating point: f32, f64
    //  char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    //  bool either true or false
    //  The unit type (), whose only possible value is an empty tuple: ()

    // Variables can always be type annotated.
    // Numbers may additionally be annotated via a suffix or by default.
    // Integers default to i32 and floats to f64.
    // Note that Rust can also infer types from context.

    // Variables can be type annotated.
    let logical: bool = true;
    println!("{}", logical);
    println!("---");

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation
    println!("{}", a_float);
    println!("{}", an_integer);
    println!("---");

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`
    println!("{}", default_float);
    println!("{}", default_integer);
    println!("---");

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    println!("{}", mutable);
    mutable = 21;
    println!("{}", mutable);

    // Variables can be overwritten with shadowing.
    // Error! The type of a variable can't be changed.
    // mutable = true;
    let mutable: bool = true;
    println!("{}", mutable);
    println!("---");

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    println!("{}", inferred_type);
    inferred_type = 4294967296i64;
    println!("{}", inferred_type);
    println!("---");
}

fn primitives_compound() {
    // Compound Types:
    //  Arrays like [1, 2, 3]
    //  Tuples like (1, true)

    // Array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", my_array);

    // Tuple is a collection of values of different types
    // and is constructed using parentheses ().
    let my_tuple: (u32, u8, bool, f32) = (5u32, 1u8, true, -5.04f32);
    println!("{:?}", my_tuple);
}

fn literals_operators() {
    // Integers 1, floats 1.2, characters 'a', strings "abc", booleans true and the unit type () can be expressed using literals.

    // Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively: 0x, 0o or 0b.
    // Underscores can be inserted in numeric literals to improve readability, e.g. 1_000 is the same as 1000, and 0.000_001 is the same as 0.000001.
    // Rust also supports scientific E-notation, e.g. 1e6, 7.6e-4. The associated type is f64.

    // We need to tell the compiler the type of the literals we use.
    // For now, we'll use the u32 suffix to indicate that the literal is an unsigned 32-bit integer, and the i32 suffix to indicate that it's a signed 32-bit integer.

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    // note that changes `1i32` to `1u32` (unsigned) would result in an error 'this arithmetic operation will overflow'
    println!("1 - 2 = {}", 1i32 - 2);

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("Boolean ops:");
    println!("\ttrue AND false is {}", true && false);
    println!("\ttrue OR false is {}", true || false);
    println!("\tNOT true is {}", !true);

    // Bitwise operations
    println!("Bitwise ops:");
    println!("\t0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("\t0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("\t0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("\t1 << 5 is {}", 1u32 << 5);
    println!("\t0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
