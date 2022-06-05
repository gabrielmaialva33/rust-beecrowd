// https://www.beecrowd.com.br/judge/pt/problems/view/1036

use std::convert::TryFrom;
use std::io;

fn main() {
    let (a, b, c) = input_line();
    let delta = b.powi(2) - 4 as f64 * a * c;
    let base = 2 as f64 * a;
    if base <= 0.0 || delta < 0.0 {
        println!("Impossivel calcular")
    } else {
        println!("R1 = {:.5}", (-b + delta.sqrt()) / base);
        println!("R2 = {:.5}", (-b - delta.sqrt()) / base)
    }
}

fn input_line() -> (f64, f64, f64) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let [x, y, z]: [String; 3] = <[String; 3]>::try_from(
        line.split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let (a, b, c) = (
        x.trim().parse::<f64>().unwrap_or_default(),
        y.trim().parse::<f64>().unwrap_or_default(),
        z.trim().parse::<f64>().unwrap_or_default(),
    );

    return (a, b, c);
}
