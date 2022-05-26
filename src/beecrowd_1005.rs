// https://www.beecrowd.com.br/judge/pt/problems/view/1005

use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();

    let a: f64 = input_a.trim().parse::<f64>().unwrap_or_default();
    let b: f64 = input_b.trim().parse::<f64>().unwrap_or_default();

    println!("MEDIA = {:.5}", ((a * 3.5) + (b * 7.5)) / 11.0)
}