// https://www.beecrowd.com.br/judge/pt/problems/view/1044

use std::convert::TryFrom;

fn main() {
    let (a, b) = input_line();
    if a % b == 0 {
        println!("Sao Multiplos")
    } else {
        println!("Nao sao Multiplos")
    }
}

fn input_line() -> (i64, i64) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut v = input
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap_or_default())
        .collect::<Vec<i64>>();
    v.sort_by(|a, b| b.cmp(a));
    let [x, y]: [i64; 2] = <[i64; 2]>::try_from(v.as_slice()).unwrap();
    return (x, y);
}
