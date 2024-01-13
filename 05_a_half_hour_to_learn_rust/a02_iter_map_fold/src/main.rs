fn main() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
    println!("{}", x);

    // let x: Vec<i32> = (1..11).collect::<Vec<_>>();
    // println!("{:?}", x);
    // let y: Vec<i32> = x.iter().map(|x| x + 3).fold(0, |x, y| x + y);
    // println!("{:?}", y);
}
