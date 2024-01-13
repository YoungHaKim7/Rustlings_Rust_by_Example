use std::convert::TryInto;

fn vector_to_array<T>(v: Vec<T>) -> [T; 10] {
    v.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!("Expected a Vec of length {} but it was {}", 10, v.len())
    })
}

fn main() {
    // let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    //     .iter()
    //     .map(|x| x + 3)
    //     .fold(0, |x, y| x + y);
    // println!("{}", x);

    let x: Vec<i32> = (1..11).collect::<Vec<_>>();
    let y = vector_to_array(x);
    println!("{:?}", &y);
    let z = y.iter().map(|x| x + 3).fold(0, |x, y| x + y);
    println!("{:?}", z);
}
