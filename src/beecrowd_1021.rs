// https://www.beecrowd.com.br/judge/pt/problems/view/1021

use std::io;

fn main() {
    let mut n: f64 = input_line();
    println!("NOTAS:");
    println!("{} nota(s) de R$ 100.00", n as i64 / 100);
    n = n % 100.0;
    println!("{} nota(s) de R$ 50.00", n as i64 / 50);
    n = n % 50.0;
    println!("{} nota(s) de R$ 20.00", n as i64 / 20);
    n = n % 20.0;
    println!("{} nota(s) de R$ 10.00", n as i64 / 10);
    n = n % 10.0;
    println!("{} nota(s) de R$ 5.00", n as i64 / 5);
    n = n % 5.0;
    println!("{} nota(s) de R$ 2.00", n as i64 / 2);
    println!("MOEDAS:");
    n = n % 2.00;
    n = n * 100.0;
    println!("{} moeda(s) de R$ 1.00", (n / 100.0) as i64);
    n = n % 100.0;
    println!("{} moeda(s) de R$ 0.50", (n / 50.0) as i64);
    n = n % 50.0;
    println!("{} moeda(s) de R$ 0.25", (n / 25.0) as i64);
    n = n % 25.0;
    println!("{} moeda(s) de R$ 0.10", (n / 10.0) as i64);
    n = n % 10.0;
    println!("{} moeda(s) de R$ 0.05", (n / 5.0) as i64);
    n = n % 5.0;
    println!("{} moeda(s) de R$ 0.01", (n / 1.0) as i64);
}

fn input_line() -> f64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().parse::<f64>().unwrap_or_default();
}
