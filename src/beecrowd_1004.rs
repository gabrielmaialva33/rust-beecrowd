// https://www.beecrowd.com.br/judge/pt/problems/view/1004

use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();

    let a: i32 = input_a.trim().parse::<i32>().unwrap_or_default();
    let b: i32 = input_b.trim().parse::<i32>().unwrap_or_default();

    println!("PROD = {}", a * b);
}