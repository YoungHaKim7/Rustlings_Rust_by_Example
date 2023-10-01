use std::mem;

struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn main() {
    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // Heap allocated varibales
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    let boxed_point: Box<Point> = Box::new(origin());

    // Bouble indirection
    let box_in_box: Box<&Box<Point>> = Box::new(&boxed_point);

    println!(
        "Point occupies {} bytes on the stack\n f64__mem align of {}",
        mem::size_of_val(&point),
        mem::align_of::<f64>()
    );
    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );
    // box size == point size
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&box_in_box)
    );
}
