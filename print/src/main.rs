fn main() {
    println!("{} days", 31);

    println!();

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!();

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!();

    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    println!();

    println!("{number:>20}", number = 1);
    println!("{number:>20}", number = 15);

    println!("{number:x>5}", number = 1); // 00001

    println!();

    #[allow(dead_code)]
    struct Structure(i32);

    let number: i32 = 20;
    let width: usize = 10;
    println!("{number:>width$}");
}
