// The enum keyword allows the creation of a type which may be one of a few different variants. Any variant which is valid as a struct is also valid in an enum.

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
pub enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
pub fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

// If you use a type alias, you can refer to each enum variant via its alias.
// This might be useful if the enum's name is too long or too generic, and you want to rename it.
#[derive(Debug)]
pub enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
pub type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// The most common place you'll see this is in impl blocks using the Self alias.
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    pub fn exec(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

pub fn example_use() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use Stage::{Advanced, Beginner};
    // Automatically `use` each name inside `Role`.
    use Role::*;

    // Equivalent to `Stage::Beginner`.
    let stage1: Stage = Beginner;
    let stage2: Stage = Advanced;

    // Equivalent to `Role::Student`.
    let role1: Role = Student;
    let role2: Role = Teacher;

    match (stage1, stage2) {
        // Note the lack of scoping because of the explicit `use` above.
        (Beginner, Beginner) => println!("Both are beginners starting their learning journey!"),
        (Beginner, Advanced) => println!("One beginner and one advanced learner!"),
        (Advanced, Beginner) => println!("One advanced learner and one beginner!"),
        (Advanced, Advanced) => println!("Both are advanced learners mastering their subjects!"),
    }

    match (role1, role2) {
        (Student, Student) => println!("Two students learning together!"),
        (Student, Teacher) => println!("A student learning from a teacher!"),
        (Teacher, Student) => println!("A teacher guiding a student!"),
        (Teacher, Teacher) => println!("Teachers collaborating!"),
    }
}

// enum can also be used as C-like enums.
enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn number_color() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    println!("grass is #{:06x}", Color::Green as i32);
}
