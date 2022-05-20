// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 }
    } = rectangle;
    (x2 - x1) * (y1 - y2)
}

fn square(top_left: Point, side_length: f32) -> Rectangle {
    let Point { x: x1, y: y1 } = top_left;
    let bottom_right = Point { x: x1 + side_length, y: y1 - side_length };

    Rectangle { top_left, bottom_right }
}

fn main() {
    // Create struct with field init shorthand
    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: 10.0, y: 10.0 },
        bottom_right: Point { x: 18.0, y: 2.0 },
    };

    println!("this should be 64: {}", rect_area(rectangle));
    println!("this should be 25: {}", rect_area(square(Point { x: 10.0, y: 10.0 }, 5.0)))
}

