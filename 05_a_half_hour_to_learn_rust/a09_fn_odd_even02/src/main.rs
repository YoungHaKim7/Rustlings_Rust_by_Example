#[derive(Debug)]
struct Number {
    odd: bool,
    value: i32,
}

fn print_number(n: Number) {
    // if let Number { odd: true, value } = n {
    //     println!("{} is odd", value);
    // } else if let Number { odd: false, value } = n {
    //     println!("{} is even", value);
    // }
    match n {
        Number { odd: true, value } => println!("Odd number: {value}"),
        Number { odd: false, value } => println!("Even number: {value}"),
    }
}

fn main() {
    let one = Number {
        odd: true,
        value: 1,
    };
    print_number(one);

    let two = Number {
        odd: false,
        value: 2,
    };

    print_number(two);
}
