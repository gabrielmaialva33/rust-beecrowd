// https://www.beecrowd.com.br/judge/pt/problems/view/1020

use std::io;

fn main() {
    let mut d = input_line();
    println!("{} ano(s)", d / 365);
    d = d % 365;
    println!("{} mes(es)", d / 30);
    d = d % 30;
    println!("{} dia(s)", d)
}

fn input_line() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().parse().unwrap_or_default();
}
