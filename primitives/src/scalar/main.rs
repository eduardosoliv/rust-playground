pub fn scalars() {
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
