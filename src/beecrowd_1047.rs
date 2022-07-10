// https://www.beecrowd.com.br/judge/pt/problems/view/1047

use std::convert::TryFrom;

fn main() {
    let (init_hour, init_min, end_hour, end_min) = input_line();

    let init = init_hour * 60 + init_min;
    let end = end_hour * 60 + end_min;

    println!(
        "O JOGO DUROU {} HORA(S) E {} MINUTO(S)",
        if init >= end {
            (((init - 1440) - end) / 60) * -1
        } else {
            (end - init) / 60
        },
        if init >= end {
            (((init - 1440) - end) % 60) * -1
        } else {
            (end - init) % 60
        }
    );
}

fn input_line() -> (i64, i64, i64, i64) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let [a, b, c, d]: [i64; 4] = <[i64; 4]>::try_from(
        input
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>(),
    )
    .unwrap();

    return (a, b, c, d);
}
