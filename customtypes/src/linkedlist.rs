pub enum List {
    // tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // signifies the end of the linked list
    Nil,
}

impl List {
    pub fn new() -> List {
        List::Nil
    }

    pub fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        List::Cons(elem, Box::new(self))
    }

    pub fn append(self, elem: u32) -> List {
        match self {
            // if no non-empty list, recursively append to the tail
            List::Cons(head, tail) => List::Cons(head, Box::new((*tail).append(elem))),
            // if an empty list, create a new single-element list
            List::Nil => List::Cons(elem, Box::new(List::Nil)),
        }
    }

    // Return the length of the list
    pub fn len(&self) -> u32 {
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
    pub fn stringify(&self) -> String {
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

    // Get value at specified index, returns None if index is out of bounds
    pub fn get(&self, index: u32) -> Option<u32> {
        match *self {
            List::Cons(head, ref tail) => {
                if index == 0 {
                    Some(head)
                } else {
                    tail.get(index - 1)
                }
            }
            List::Nil => None,
        }
    }

    pub fn get_or_fail(&self, index: u32) -> u32 {
        match self.get(index) {
            Some(value) => value,
            None => panic!("value for index {} not found", index),
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
    println!("value for position 1: {}", list.get_or_fail(1));
}
