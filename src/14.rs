
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("savage_tiger", "A savage tiger");
    println!("{}", map["savage_tiger"]);
}