// A common way to implement a linked-list is via enums:

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        List::Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        List::Cons(elem, Box::new(self))
    }

    // Consume a list, and return the same list with a new element at its end
    fn append(self, elem: u32) -> List {
        match self {
            // If we have a non-empty list, recursively append to the tail
            List::Cons(head, tail) => List::Cons(head, Box::new((*tail).append(elem))),
            // If we have an empty list, create a new single-element list
            List::Nil => List::Cons(elem, Box::new(List::Nil)),
        }
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            List::Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            List::Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            List::Nil => {
                format!("Nil")
            }
        }
    }
}

pub fn list() {
    let mut list: List = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    let mut list2: List = List::new();
    list2 = list2.append(1);
    list2 = list2.append(2);
    list2 = list2.append(3);

    println!("linked list has length: {}", list2.len());
    println!("{}", list2.stringify());
}
