fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}, this is {2}", "Alice", "Bob", "GYoung");

    // As can named arguments
    println!(
        "{subject} {verb} {object}",
        subject = "Alice",
        verb = "is",
        object = "a"
    );

    println!("Base 10:          {}", 69420);
    println!("Base 2 (Binary): {}", 0b101010);
    println!("Base 16 (Hex):    {}", 0x12);
    println!("Base 8 (Octal):    {}", 0o77);

    println!("{number:>5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);

    #[derive(Debug)]
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}", number = number, width = width);
    let structure = Structure(1);
    println!("{structure:?}");
}
