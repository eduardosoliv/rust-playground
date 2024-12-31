// Compound Types:
//  Arrays like [1, 2, 3]
//  Tuples like (1, true)

pub fn array() {
    // Array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", my_array);
}

pub fn tuple() {
    // Tuple is a collection of values of different types
    // and is constructed using parentheses ().
    let my_tuple: (u32, u8, bool, f32) = (5u32, 1u8, true, -5.04f32);
    println!("{:?}", my_tuple);
}
