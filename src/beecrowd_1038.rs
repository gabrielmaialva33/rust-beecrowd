// https://www.beecrowd.com.br/judge/pt/problems/view/1038

use std::convert::TryFrom;
use std::io;

fn main() {
    let (code, quantity) = input_line();
    match code {
        1 => {
            println!("Total: R$ {:.2}", 4.00 * quantity as f64)
        }
        2 => {
            println!("Total: R$ {:.2}", 4.50 * quantity as f64)
        }
        3 => {
            println!("Total: R$ {:.2}", 5.00 * quantity as f64)
        }
        4 => {
            println!("Total: R$ {:.2}", 2.00 * quantity as f64)
        }
        5 => {
            println!("Total: R$ {:.2}", 1.50 * quantity as f64)
        }
        _ => {}
    }
}

fn input_line() -> (i64, i64) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let [x, y]: [String; 2] = <[String; 2]>::try_from(
        line.split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let (code, quantity) = (
        x.trim().parse::<i64>().unwrap_or_default(),
        y.trim().parse::<i64>().unwrap_or_default(),
    );

    return (code, quantity);
}
