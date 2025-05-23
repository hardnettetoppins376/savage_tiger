use std::collections::VecDeque;

fn main() {
    let mut numbers: VecDeque<i32> = VecDeque::new();
    numbers.push_back(1);
    numbers.push_back(2);
    numbers.push_back(3);

    for &number in &numbers {
        println!("{}", number);
    }
}
