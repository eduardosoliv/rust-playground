// Similar to if let, while let can make awkward match sequences more tolerable.
// Consider the following sequence that increments i:

#[allow(dead_code)]
fn temp() {
    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ Requires 3 indentations!
            }
            _ => {
                break;
            } // ^ Why should this be required? There must be a better way!
        }
    }
}

pub fn while_let_example(mut optional: Option<i32>) {
    // ^ Less rightward drift and doesn't require
    // explicitly handling the failing case.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
