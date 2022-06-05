// https://www.beecrowd.com.br/judge/pt/problems/view/1035

use std::convert::TryFrom;
use std::io;

fn main() {
    let (a, b, c, d) = input_line();
    if b > c && d > a && (c + d) > (a + b) && c > 0 && d > 0 && a % 2 == 0 {
        println!("Valores aceitos")
    } else {
        println!("Valores nao aceitos")
    }
}

fn input_line() -> (i64, i64, i64, i64) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let [w, x, y, z]: [String; 4] = <[String; 4]>::try_from(
        line.split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let (a, b, c, d): (i64, i64, i64, i64) = (
        w.parse::<i64>().unwrap_or_default(),
        x.parse::<i64>().unwrap_or_default(),
        y.parse::<i64>().unwrap_or_default(),
        z.parse::<i64>().unwrap_or_default(),
    );

    return (a, b, c, d);
}
