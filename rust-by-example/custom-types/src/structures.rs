struct Nil;

struct Pair(i32, f32);

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn square(p: Point, t: f32) -> Rectangle {
    Rectangle {p1: p, p2: Point {x: p.x + t, y: p.y + t}}
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle{p1: Point{x: x1, y: y1}, p2: Point{x: x2, y: y2}} = rect;
    (x1 - x2) * (y1 - y2)
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the field of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an exporession too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle{p1: Point{x: 0.0, y: 0.0}, p2: Point {x: 5.0, y: 5.0}};
    println!("rect_area {:?}", rect_area(rect));

    println!("square {:?}", square(point, 3.0))
}
