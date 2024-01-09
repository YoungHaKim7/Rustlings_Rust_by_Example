use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("{:?}", minmax);
    println!("{:?}", minmax.0);
    println!("{:?}", minmax.1);

    println!("Compare values:");
    println!("{}", minmax.0);
    println!("{}", minmax.1);

    println!("Compare references:");
    let minmax = &minmax;
    println!("{}", minmax.0);
    println!("{}", minmax.1);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(0, 10);

    println!(
        "The big rage is {big:?} and the small range is {small}.",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 10., y: 20. };

    println!("Compare values: ");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}
