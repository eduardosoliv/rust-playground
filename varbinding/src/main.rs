fn main() {
    mutability();
    println!("------");
    scope();
    println!("------");
    shadowing();
    println!("------");
    declare_first();
    println!("------");
    freezing();
}

fn mutability() {
    let _immutable_binding: i32 = 1;
    let mut mutable_binding: i32 = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to an immutable variable
    // _immutable_binding += 1;
}

fn scope() {
    // lives in scope function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}

fn shadowing() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outside inner block: {}", shadowed_binding);
}

fn declare_first() {
    let a_binding;
    {
        a_binding = 2 * 2;
    }
    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

fn freezing() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
    println!("mutable integer: {}", _mutable_integer);
}
