// https://www.beecrowd.com.br/judge/pt/problems/view/1015

use std::convert::TryFrom;
use std::io;

fn main() {
    let (mut x1, mut x2, mut y1, mut y2) = (0.0, 0.0, 0.0, 0.0);
    for i in 0..2 {
        let (a, b) = input_line();
        match i {
            0 => {
                x1 = a;
                y1 = b
            }
            1 => {
                x2 = a;
                y2 = b
            }
            _ => {}
        }
    }

    println!("{:.4}", ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt())
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

    let (x, y): (f64, f64) = (
        a.parse::<f64>().unwrap_or_default(),
        b.parse::<f64>().unwrap_or_default(),
    );

    return (x, y);
}
