// https://www.beecrowd.com.br/judge/pt/problems/view/1012

use std::convert::TryFrom;
use std::io;

fn main() {
    let (a, b, c) = input_line();

    println!("TRIANGULO: {:.3}", (a * c) / 2.0);
    println!("CIRCULO: {:.3}", (3.14159) * c.powi(2));
    println!("TRAPEZIO: {:.3}", ((a + b) * c) / 2.0);
    println!("QUADRADO: {:.3}", b.powi(2));
    println!("RETANGULO: {:.3}", a * b)
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

    let a: f64 = x.trim().parse::<f64>().unwrap_or_default();
    let b: f64 = y.trim().parse::<f64>().unwrap_or_default();
    let c: f64 = z.trim().parse::<f64>().unwrap_or_default();

    return (a, b, c);
}
