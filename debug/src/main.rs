struct UnPrintable(i32);

#[derive(Debug)]
struct Printable(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    print!("UnPrintable: ");
    let unp: UnPrintable = UnPrintable(3);
    println!("{:?}", unp.0);

    println!();

    println!("Printable:");
    let p: Printable = Printable(3);
    println!("{:?}", p);
    println!("{:?}", p.0);

    println!();

    let name: &str = "Peter";
    let age: u8 = 27;
    let person1: Person<'_> = Person { name, age };

    // Pretty print
    println!("{:#?}", person1);
    println!("{name} {age}", name = person1.name, age = person1.age);
}
