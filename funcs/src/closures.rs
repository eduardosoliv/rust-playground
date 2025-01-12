// Closures are functions that can capture the enclosing environment.
// For example, a closure that captures the x variable:
// |val| val + x

// The syntax and capabilities of closures make them very convenient for on the fly usage.
// Calling a closure is exactly like calling a function.
// However, both input and return types can be inferred and input variable names must be specified.

// Other characteristics of closures include:
//  using || instead of () around input variables.
//  optional body delimitation ({}) for a single line expression (mandatory otherwise).
//  the ability to capture the outer environment variables.

pub fn closures_example() {
    let outer_var = 42;

    // Error, A regular function can't refer to variables in the enclosing environment
    // fn function(i: i32) -> i32 { i + outer_var }

    // Closures are anonymous, here we are binding them to references.
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));

    println!("closure_inferred: {}", closure_inferred(1));
    // Once closure's type has been inferred, it cannot be inferred again with another type.
    // println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one())
}

// CAPTURING

// Closures are inherently flexible and will do what the functionality requires to make the closure work without annotation.
// This allows capturing to flexibly adapt to the use case, sometimes moving and sometimes borrowing.
// Closures can capture variables:
//  by reference: &T
//  by mutable reference: &mut T
//  by value: T
// They preferentially capture variables by reference and only go lower when required.

pub fn closures_capturing() {
    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // A move or reborrow is *NOT* allowed until the final use of `print`
    // Uncommeting next line results on: cannot move out of `color` because it is borrowed
    // let _color_moved = color;

    // Since the closure `print` only takes an immutable reference to `color` when it's called,
    // we can still create another immutable reference to `color` here.
    // This demonstrates that multiple immutable references are allowed.
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // Uncommenting the next line throws an Error: cannot borrow `count` as immutable because it is also borrowed as mutable
    // let _reborrow = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let count_reborrowed = &mut count;

    println!("`count`: {}", count_reborrowed);

    // Box<T> is a smart pointer that stores data on the heap rather than the stack.
    // It's useful when you have a type of unknown size at compile time or when you
    // want to transfer ownership of a large amount of data without copying it.
    // Here we create a Box containing the integer 3, which gets stored on the heap.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        std::mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.
}

pub fn closures_capturing_move() {
    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // Uncommenting below line will result in compile-time error because
    // borrow checker doesn't allow re-using variable after it has been moved.
    // println!("There're {} elements in vec", haystack.len());
}

pub fn closures_capturing_copy() {
    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    // Removing `move` from closure's signature will cause closure to borrow
    // `haystack` variable immutably, hence `haystack` is still available
    let contains = |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    println!("There're {} elements in vec", haystack.len());
}
