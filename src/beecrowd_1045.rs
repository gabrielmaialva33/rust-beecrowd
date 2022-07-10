// https://www.beecrowd.com.br/judge/pt/problems/view/1045

use std::convert::TryFrom;

fn main() {
    let (a, b, c) = input_line();

    if a >= b + c {
        println!("NAO FORMA TRIANGULO")
    }
    if !(a >= b + c) && (a.powf(2.0) == (b.powf(2.0) + c.powf(2.0))) {
        println!("TRIANGULO RETANGULO")
    }
    if !(a >= b + c) && (a.powf(2.0) > (b.powf(2.0) + c.powf(2.0))) {
        println!("TRIANGULO OBTUSANGULO")
    }
    if !(a >= b + c) && (a.powf(2.0) < (b.powf(2.0) + c.powf(2.0))) {
        println!("TRIANGULO ACUTANGULO")
    }
    if !(a >= b + c) && (a == b && b == c && a == c) {
        println!("TRIANGULO EQUILATERO")
    }
    if !(a >= b + c) && (a == b || b == c || c == a) && !(a == b && b == c && a == c) {
        println!("TRIANGULO ISOSCELES")
    }
}

fn input_line() -> (f64, f64, f64) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut v = input
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let [a, b, c]: [f64; 3] = <[f64; 3]>::try_from(v.as_slice()).unwrap();
    return (a, b, c);
}
