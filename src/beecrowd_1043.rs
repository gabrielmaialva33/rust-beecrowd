// https://www.beecrowd.com.br/judge/pt/problems/view/1042

use std::convert::TryFrom;

fn main() {
    let (a, b, c) = input_line();

    if a + b > c && a + c > b && b + c > a {
        println!("Perimetro = {:.1}", a + b + c);
    } else {
        println!("Area = {:.1}", (a + b) * (c / 2.0));
    }
}

fn input_line() -> (f64, f64, f64) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    let [x, y, z]: [String; 3] = <[String; 3]>::try_from(
        input
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let (a, b, c) = (
        x.trim().parse::<f64>().unwrap_or_default(),
        y.trim().parse::<f64>().unwrap_or_default(),
        z.trim().parse::<f64>().unwrap_or_default(),
    );

    return (a, b, c);
}
