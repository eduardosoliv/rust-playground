fn main() {
    mutability();
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
