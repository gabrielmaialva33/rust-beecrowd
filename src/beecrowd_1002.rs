// https://www.beecrowd.com.br/judge/pt/problems/view/1002

use std::io;

fn main() {
    let mut input_r = String::new();

    io::stdin().read_line(&mut input_r).unwrap();

    let r: f64 = input_r.trim().parse::<f64>().unwrap_or_default();

    println!("A={:.4}", 3.14159 * (r.powi(2)))
}