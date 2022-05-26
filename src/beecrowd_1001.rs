use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    let _ = io::stdin().read_line(&mut input_a);
    let _ = io::stdin().read_line(&mut input_b);

    let a: i32 = input_a.trim().parse::<i32>().unwrap_or_default();
    let b: i32 = input_b.trim().parse::<i32>().unwrap_or_default();

    println!("X = {}", a + b);
}
