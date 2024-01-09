fn main() {
    // arguments. These will be stringfied.
    let args: Vec<String> = std::env::args().collect();

    // Print the arguments.
    for arg in args {
        println!("{}", arg);
    }

    println!("Hello, world!");
}
