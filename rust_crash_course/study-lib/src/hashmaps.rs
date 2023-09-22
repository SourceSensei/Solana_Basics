use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(0, "Hi");
    map.insert(1, "Hi again");
    println!("{:?}", map);

    match map.get(&0) {
        Some(str1) => println!("{}", str1),
        None => println!("Doesn't exist in map"),
    }
}