mod structures;
use structures::{person, points_rectangle, square, unit_pair, Point, Rectangle};

mod enums;
use enums::{example_use, inspect, number_color, Operations, WebEvent};

mod linkedlist;
use linkedlist::list;

fn main() {
    person();
    println!("------");
    points_rectangle();
    println!("------");
    unit_pair();
    println!("------");
    let point: Point = Point { x: 5.2, y: 0.4 };
    let width_height: f32 = 20.0;
    println!("Creating rectangle with  top left corner point: {:?} and width and height corresponding to {}", point, width_height);
    let rectlangle: Rectangle = square(point, width_height);
    println!("{:?}", rectlangle);

    println!("------------------");

    let pressed: WebEvent = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted: WebEvent = WebEvent::Paste("my text".to_owned());
    let click: WebEvent = WebEvent::Click { x: 20, y: 80 };
    let load: WebEvent = WebEvent::PageLoad;
    let unload: WebEvent = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    println!("------");

    let add: Operations = Operations::Add;
    println!("{:?}", add);
    println!("{} + {} = {}", 50, 20, add.exec(50, 20));

    let subtract: Operations = Operations::Subtract;
    println!("{:?}", subtract);
    println!("{} - {} = {}", 50, 20, subtract.exec(50, 20));

    println!("------");

    example_use();

    println!("------");

    number_color();

    println!("------------------");

    list();
}
