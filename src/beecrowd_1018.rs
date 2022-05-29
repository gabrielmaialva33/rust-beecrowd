// https://www.beecrowd.com.br/judge/pt/problems/view/1018

use std::io;

fn main() {
    let mut n: i32 = input_line();
    println!("{}", n);
    println!("{} nota(s) de R$ 100,00", n / 100);
    n = n % 100;
    println!("{} nota(s) de R$ 50,00", n / 50);
    n = n % 50;
    println!("{} nota(s) de R$ 20,00", n / 20);
    n = n % 20;
    println!("{} nota(s) de R$ 10,00", n / 10);
    n = n % 10;
    println!("{} nota(s) de R$ 5,00", n / 5);
    n = n % 5;
    println!("{} nota(s) de R$ 2,00", n / 2);
    n = n % 2;
    println!("{} nota(s) de R$ 1,00", n / 1);
}

fn input_line() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().parse::<i32>().unwrap_or_default();
}
