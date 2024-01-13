fn main() {
    let slice = "Hello, world!";
    let (left, right) = slice.split_at(5);
    println!("left: {}", left);
    println!("right: {}", right);

    let slice02 = "Hello, world!";
    let len = slice02.len();
    let middle = (len) / 2;

    let (left, right) = slice.split_at(middle);
    println!("left: {}", left);
    println!("right: {}", right);
}
