// https://www.beecrowd.com.br/judge/pt/problems/view/1041

use std::convert::TryFrom;
use std::io;

fn main() {
    let (x, y) = input_line();

    if x > 0.0 && y > 0.0 {
        println!("Q1");
    } else if x > 0.0 && y < 0.0 {
        println!("Q4");
    } else if x < 0.0 && y > 0.0 {
        println!("Q2");
    } else if x < 0.0 && y < 0.0 {
        println!("Q3");
    } else if (x < 0.0 || x > 0.0) && y == 0.0 {
        println!("Eixo X");
    } else if x == 0.0 && (y < 0.0 || y > 0.0) {
        println!("Eixo Y");
    } else {
        println!("Origem");
    }
}

fn input_line() -> (f64, f64) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let [a, b]: [String; 2] = <[String; 2]>::try_from(
        line.split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let (x, y) = (
        a.trim().parse::<f64>().unwrap_or_default(),
        b.trim().parse::<f64>().unwrap_or_default(),
    );

    return (x, y);
}
