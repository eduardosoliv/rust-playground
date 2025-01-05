// The for in construct can be used to iterate through an Iterator.
// One of the easiest ways to create an iterator is to use the range notation a..b.
// This yields values from a (inclusive) to b (exclusive) in steps of one.

pub fn fizzbuzz_forloop() {
    // from 1 to 20, is exclusive int the right side
    for n in 1..21 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

pub fn fizzbuzz_forloop_inclusive() {
    // from 1 to 20, as = makes it inclusive in the right side
    for n in 1..=20 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

// The for in construct is able to interact with an Iterator in several ways.
// As discussed in the section on the Iterator trait, by default the for loop will apply the into_iter function to the collection.
// However, this is not the only means of converting collections into iterators.

// into_iter, iter and iter_mut all handle the conversion of a collection into an iterator in different ways,
// by providing different views on the data within.

pub fn for_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // iter - This borrows each element of the collection through each iteration.
    // Thus leaving the collection untouched and available for reuse after the loop.
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}

pub fn for_into_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // into_iter - This consumes the collection so that on each iteration the exact data is provided.
    // Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // Once the collection has been consumed it is no longer available for reuse
    // as such the line below needs to be commented:
    // println!("names: {:?}", names);
}

pub fn for_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => &name,
        }
    }

    println!("names: {:?}", names);
}
