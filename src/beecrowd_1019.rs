// https://www.beecrowd.com.br/judge/pt/problems/view/1019

use std::io;

fn main() {
    let t = input_line();

    println!("{}:{}:{}", t / 3600, (t % 3600) / 60, ((t % 3600) % 60))
}

fn input_line() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().parse().unwrap_or_default();
}
