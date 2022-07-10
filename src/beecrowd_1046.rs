// https://www.beecrowd.com.br/judge/pt/problems/view/1046

use std::convert::TryFrom;

fn main() {
    let (init_hour, end_hour) = input_line();

    println!(
        "O JOGO DUROU {} HORA(S)",
        if init_hour >= end_hour {
            ((init_hour - 24) - end_hour) * -1
        } else {
            end_hour - init_hour
        }
    );
}

fn input_line() -> (i64, i64) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let [x, y]: [i64; 2] = <[i64; 2]>::try_from(
        input
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>(),
    )
    .unwrap();

    return (x, y);
}
