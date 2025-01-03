// Primitive types can be converted to each other through casting.

// Rust addresses conversion between custom types (i.e., struct and enum) by the use of traits.
// The generic conversions will use the From and Into traits.
// However there are more specific ones for the more common cases, in particular when converting to and from Strings.

// From and Into
// The From and Into traits are inherently linked, and this is actually part of its implementation.
// If you are able to convert type A from type B, then it should be easy to believe that we should be able to convert type B to type A.

// From
// The From trait allows for a type to define how to create itself from another type.
// Hence providing a very simple mechanism for converting between several types.
// There are numerous implementations of this trait within the standard library for conversion of primitive and common types.
use std::convert::{From, Into};
use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// Rust automatically implements Into for any type that implements From.
// Note, however, that the converse is not true: implementing Into for your type will not automatically provide it with an implementation of From.
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
struct Name {
    first: String,
    last: String,
}

impl From<&str> for Name {
    fn from(name: &str) -> Self {
        let parts: Vec<&str> = name.split_whitespace().collect();

        let first = parts.first().unwrap_or(&"").to_string();
        let mut last = String::from("");
        if parts.len() > 1 {
            last = parts.last().unwrap_or(&"").to_string();
        }

        Name { first, last }
    }
}

pub fn from() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    println!("{}", my_string);

    // We can do something similar for defining a conversion for our own type.
    let num = Number::from(30);
    println!("My number is {}", num);
    println!("");

    // another example
    let str = "John Doe";
    println!("{:?}", str);
    let name = Name::from(str);
    println!("{:?}", name);
    println!("first: {} last: {}", name.first, name.last);
    println!("");

    // empty string
    let str = "";
    println!("{:?}", str);
    let empty_name = Name::from(str);
    println!("{:?}", empty_name);
    println!("first: {} last: {}", empty_name.first, empty_name.last);
    println!("");

    // single name
    let str = "John";
    println!("{:?}", str);
    let two_names = Name::from(str);
    println!("{:?}", two_names);
    println!("first: {} last: {}", two_names.first, two_names.last);
    println!("");

    // 3 names
    let str = "John Oliver Doe";
    println!("{:?}", str);
    let three_names = Name::from(str);
    println!("{:?}", three_names);
    println!("first: {} last: {}", three_names.first, three_names.last);
    println!("");
}

pub fn into() {
    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
