use std::collections::HashMap;

fn main() {
    let mut data: HashMap<String, i32> = HashMap::new();

    // Example data
    data.insert(String::from("key1"), 5);
    data.insert(String::from("key2"), 10);
    data.insert(String::from("key3"), 7);

    for (key, value) in &data {
        println!("Key: {}", key);
        println!("Value: {}", value);
    }
}
