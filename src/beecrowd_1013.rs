// https://www.beecrowd.com.br/judge/pt/problems/view/1013

use std::convert::TryFrom;
use std::io;

fn main() {
    let (a, b, c) = input_line();

    println!("{} eh o maior", vec![a, b, c].iter().max().unwrap())
}

fn input_line() -> (i32, i32, i32) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap_or_default();

    let [x, y, z]: [String; 3] = <[String; 3]>::try_from(
        line.split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let a: i32 = x.trim().parse::<i32>().unwrap_or_default();
    let b: i32 = y.trim().parse::<i32>().unwrap_or_default();
    let c: i32 = z.trim().parse::<i32>().unwrap_or_default();

    return (a, b, c);
}
