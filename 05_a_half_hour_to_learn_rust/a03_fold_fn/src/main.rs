fn main() {
    let numbers = [1, 2, 3, 4, 5];

    let mut result = 0;

    // for loop:
    for i in &numbers {
        result = result + i;
    }
    println!("result : {}", result);

    // fold:
    let result2 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("result2 : {}", result2);
}
