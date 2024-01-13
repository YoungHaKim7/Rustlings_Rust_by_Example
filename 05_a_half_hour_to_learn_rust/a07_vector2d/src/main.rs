#[derive(Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

fn main() {
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 3.0, y: 4.0 };

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);

    let v3 = Vec2 { x: 5.0, ..v2 };
    println!("v3 = {:?}", v3);
}
