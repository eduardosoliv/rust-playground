// There are three types of structures ("structs") that can be created using the struct keyword:
//      Tuple structs, which are, basically, named tuples.
//      The classic C structs
//      Unit structs, which are field-less, are useful for generics.

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
// Structs can be reused as fields of another struct
pub struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    pub tleft: Point,
    pub bright: Point,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

pub fn person() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);
    println!(
        "name: {name} age: {age}",
        name = peter.name,
        age = peter.age
    );
}

pub fn points_rectangle() {
    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right: Point = Point {
        x: 10.3,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    println!(
        "destructed point: left_edge: {}, top_edge: {}",
        left_edge, top_edge
    );

    let rectangle: Rectangle = Rectangle {
        // struct instantiation is an expression too
        tleft: Point {
            x: left_edge,
            y: top_edge,
        },
        bright: bottom_right,
    };

    println!("rectangle: {:?}", rectangle);
    println!(
        "rectangle top left: ({}, {})",
        rectangle.tleft.x, rectangle.tleft.y
    );
    println!(
        "rectangle bottom right: ({}, {})",
        rectangle.bright.x, rectangle.bright.y
    );

    println!("area of rectangle (rounded): {:.2}", rect_area(rectangle));
}

pub fn unit_pair() {
    let _unit: Unit = Unit;

    let pair: Pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn rect_area(rectangle: Rectangle) -> f32 {
    // Destructure the rectangle into nested Point components
    let Rectangle {
        tleft: Point { x: x1, y: y1 },
        bright: Point { x: x2, y: y2 },
    } = rectangle;

    // Calculate width and height
    let width: f32 = (x2 - x1).abs();
    let height: f32 = (y2 - y1).abs();

    // Calculate and return area
    width * height
}

pub fn square(point: Point, width_height: f32) -> Rectangle {
    let bottom_right_x: f32 = point.x + width_height;
    let bottom_right_y: f32 = point.y + width_height;

    let rectangle: Rectangle = Rectangle {
        // struct instantiation is an expression too
        tleft: point,
        bright: Point {
            x: bottom_right_x,
            y: bottom_right_y,
        },
    };

    rectangle
}
