// https://www.beecrowd.com.br/judge/pt/problems/view/1016

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let k = line.trim().parse::<i32>().unwrap_or_default();
    println!("{} minutos", k * 2)
}
