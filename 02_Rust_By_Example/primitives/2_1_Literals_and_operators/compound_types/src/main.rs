// Compound Types
// arrays like [1,2,3]
// Tuples like (1, true)

fn main() {
    // Variables can be type annotated.
    let x = 1;
    let y = 2;
    let z = "Hello, world!";
    println!("result: {}", z);

    // You can also use variables as function parameters.
    let add = |x: i32, y: i32| x + y;
    let result = add(x, y);
    println!("result: {}", result);

    // You can also use variables as function return values.
    let multiply = |x: i32, y: i32| x * y;
    let result02 = multiply(x, y);
    println!("result: {} ", result02);

    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer: i32 = 5i32;

    // Or a default will be used.
    let default_float = 3.0;
    let default_integer = 7;

    // A type can also be inferred from context.
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable: bool = false;

    mutable = true;
    dbg!(mutable);
}
