use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 4);
    basket.insert(String::from("pineapple"), 1);
    basket.insert(String::from("melon"), 8);

    basket
}

fn main() {
    println!("{:?}", fruit_basket());
}
