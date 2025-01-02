mod structures;
use structures::{person, points_rectangle, square, unit_pair, Point, Rectangle};

fn main() {
    person();
    println!("---");
    points_rectangle();
    println!("---");
    unit_pair();
    println!("---");
    let point: Point = Point { x: 5.2, y: 0.4 };
    let width_height: f32 = 20.0;
    println!("Creating rectangle with  top left corner point: {:?} and width and height corresponding to {}", point, width_height);
    let rectlangle: Rectangle = square(point, width_height);
    println!("{:?}", rectlangle);
}
