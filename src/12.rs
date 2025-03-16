fn main() {
    let x = 1;
    let y = 2;
    println!("{} + {} = {}", x, y, sum(x, y));
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
