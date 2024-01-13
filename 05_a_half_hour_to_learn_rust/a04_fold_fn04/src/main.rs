use std::{convert::TryInto, error::Error};

fn vector_to_array<T, const N: usize>(v: Vec<T>) -> Result<[T; N], Box<dyn Error>> {
    Ok(v.try_into()
        .map_err(|v: Vec<T>| format!("Expected a Vec of length {} but it was {}", N, v.len()))?)
}

fn main() {
    let x: Vec<i32> = (1..11).collect::<Vec<_>>();

    // Specify the expected array length as a constant
    let y: [i32; 10] = vector_to_array(x).unwrap();
    println!("{:?}", &y);

    let z = y.iter().map(|x| x + 3).fold(0, |x, y| x + y);
    println!("{}", z);
}
