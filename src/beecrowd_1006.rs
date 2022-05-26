// https://www.beecrowd.com.br/judge/pt/problems/view/1006

use std::io;

fn main() {
    let (mut input_a, mut input_b, mut input_c) = (String::new(), String::new(), String::new());

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    io::stdin().read_line(&mut input_c).unwrap();

    let a: f64 = input_a.trim().parse::<f64>().unwrap_or_default();
    let b: f64 = input_b.trim().parse::<f64>().unwrap_or_default();
    let c: f64 = input_c.trim().parse::<f64>().unwrap_or_default();

    println!("MEDIA = {:.1}", ((a * 2.0) + (b * 3.0) + (c * 5.0)) / 10.0)
}