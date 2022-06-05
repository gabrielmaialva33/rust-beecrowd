// https://www.beecrowd.com.br/judge/pt/problems/view/1020

use std::io;

fn main() {
    let mut t = input_line();
    println!("{} ano(s)", t / 365);
    t = t % 365;
    println!("{} mes(es)", t / 30);
    t = t % 30;
    println!("{} dia(s)", t)
}

fn input_line() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().parse().unwrap_or_default();
}
