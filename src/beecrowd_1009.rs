// https://www.beecrowd.com.br/judge/pt/problems/view/1009

use std::io;

fn main() {
    let (y, z) = input();
    println!("TOTAL = R$ {:.2}", y + ((z * 15 as f64) / 100 as f64))
}

fn input() -> (f64, f64) {
    let (mut name, mut salary, mut amount): (String, String, String) =
        (String::new(), String::new(), String::new());

    io::stdin().read_line(&mut name).unwrap();
    io::stdin().read_line(&mut salary).unwrap();
    io::stdin().read_line(&mut amount).unwrap();

    let (y, z): (f64, f64) = (
        salary.trim().parse::<f64>().unwrap_or_default(),
        amount.trim().parse::<f64>().unwrap_or_default(),
    );

    return (y, z);
}
