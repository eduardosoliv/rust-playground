// To convert any type to a String is as simple as implementing the ToString trait for the type.
// Rather than doing so directly, you should implement the fmt::Display trait which automatically
// provides ToString and also allows printing the type as discussed in the section on print!.

use std::fmt;

struct Circle {
    radius: i32,
}

pub struct Name {
    pub first: String,
    pub middle: String,
    pub last: String,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<&str> = Vec::with_capacity(3);
        for part in [&self.first, &self.middle, &self.last] {
            if !part.is_empty() {
                parts.push(part);
            }
        }
        write!(f, "{}", parts.join(" "))
    }
}

pub fn circle() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

pub fn name() {
    let name1 = Name {
        first: String::from("John"),
        middle: String::from("C"),
        last: String::from("Doe"),
    };
    println!("{}", name1.to_string());

    let name2 = Name {
        first: String::from("John"),
        middle: String::from(""),
        last: String::from("Doe"),
    };
    println!("{}", name2.to_string());
}
