// Rust provides a loop keyword to indicate an infinite loop.

// The break statement can be used to exit a loop at anytime
// whereas the continue statement can be used to skip the rest of the iteration and start a new one.

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

#[allow(unreachable_code)]
#[allow(unused_labels)]
pub fn loop_nesting() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

pub fn loop_assign() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("{}", result);
}
