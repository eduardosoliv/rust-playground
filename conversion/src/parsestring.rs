// It's useful to convert strings into many types, but one of the more common string operations is to convert them from string to number.
// The idiomatic approach to this is to use the parse function and either to arrange for type inference or to specify the type to parse using the 'turbofish' syntax.
// Both alternatives are shown in the following example.

// This will convert the string into the type specified as long as the FromStr trait is implemented for that type.
// This is implemented for numerous types within the standard library.
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(e) => Err(e),
        }
    }
}

pub fn parse_string() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    println!("Sum: {:?}", parsed + turbo_parsed);
}

pub fn parse_from_string(radius: &str) {
    let parse_result = radius.parse();
    if parse_result.is_err() {
        println!("Failed to parse radius: {:?}", parse_result.err().unwrap());
        return;
    }

    let circle: Circle = parse_result.unwrap();
    println!("{:?}", circle);
    println!("circle radius {}", circle.radius);
}
