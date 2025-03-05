// Savage Tiger
fn main() {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..=10);
    println!("The random number is {}", number);
}
