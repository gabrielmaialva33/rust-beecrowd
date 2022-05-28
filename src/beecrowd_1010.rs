// https://www.beecrowd.com.br/judge/pt/problems/view/1010

use std::convert::TryFrom;
use std::io;

fn main() {
    let mut total: f64 = 0.0;
    for _ in 0..2 {
        total += line_input()
    }
    println!("VALOR A PAGAR: R$ {:.2}", total)
}

fn line_input() -> f64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap_or_default();

    let [_, x, y]: [String; 3] = <[String; 3]>::try_from(
        line.split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let parts = x.parse::<i32>().unwrap_or_default();
    let value = y.parse::<f64>().unwrap_or_default();

    return parts as f64 * value;
}
