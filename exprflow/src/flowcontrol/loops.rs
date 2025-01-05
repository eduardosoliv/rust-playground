pub fn simple_loop() {
    let mut count: u32 = 0;

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            // skip the code below
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }
}
