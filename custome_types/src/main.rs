// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
fn rect_area(rect: Rectangle) -> f32 { 
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 }
    } = rect;
    let width = (x1 -x2).abs();
    let height = (y1 - y2).abs();

    width * height
}

// Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.
fn square(corner1: Point, length: f32) -> Rectangle {
    let corner2: Point = Point {x: corner1.x + length, y: corner1.y + length};
    Rectangle {top_left: corner1, bottom_right: corner2}

}
    


fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    let another_point: Point = Point { x: 5.2, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..another_point };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let rect = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Implement rect_area
    let area: f32 = rect_area(rect);
    println!("Are of the rectangle is {}", area);

    // Implement square
    let square_corner: Point = Point {x: 2.0, y: 4.0};
    let new_rect: Rectangle = square(square_corner, 3.2);
    println!("The square is {:?}", new_rect);

}