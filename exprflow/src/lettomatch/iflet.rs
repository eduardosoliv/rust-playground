// For some use cases, when matching enums, match is awkward. For example:

pub fn awkward_match(option: Option<i32>) {
    match option {
        Some(i) => println!("This is a really long string and `{:?}`", i),
        _ => {} // ^ Required because `match` is exhaustive. Doesn't it seem like wasted space?
    };
}

pub fn simple_let(number: Option<i32>) {
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
}

pub fn let_else(letter: Option<i32>) {
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }
}

pub fn let_if_else() {
    let emoticon: Option<i32> = None;

    // Provide an altered failing condition.
    let i_like_letters = true;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

pub fn if_let_match_enum() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        // no match will not print
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(_value @ 100) = c {
        println!("c is one hundred");
    }
}
