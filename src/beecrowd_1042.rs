// https://www.beecrowd.com.br/judge/pt/problems/view/1041

use std::convert::TryFrom;

fn main() {
    let (x, y, z) = input_line();
    let mut array: [i64; 3] = [x, y, z];
    array.sort();

    for i in array {
        println!("{}", i);
    }

    println!();

    let array: [i64; 3] = [x, y, z];
    for i in array {
        println!("{}", i);
    }
}

fn input_line() -> (i64, i64, i64) {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let [a, b, c]: [String; 3] = <[String; 3]>::try_from(
        line.split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let (x, y, z) = (
        a.trim().parse::<i64>().unwrap_or_default(),
        b.trim().parse::<i64>().unwrap_or_default(),
        c.trim().parse::<i64>().unwrap_or_default(),
    );

    return (x, y, z);
}
