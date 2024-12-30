use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing, and create a reference to `vec`.
        let vec: &Vec<i32> = &self.0;

        // The ? operator is used for error propagation
        // It will return the error if write! fails, otherwise continue execution
        write!(f, "[")?;

        // Iterate over `v` in `vec` count in `count` while enumerating the iteration
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            if count != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v: List = List(vec![1, 2, 3]);

    println!("{}", v);
}
