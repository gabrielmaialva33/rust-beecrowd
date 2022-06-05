// https://www.beecrowd.com.br/judge/pt/problems/view/1037

use std::io;

fn main() {
    let x = input_line();
    if x >= 0.0 && x <= 25.0 {
        println!("Intervalo [0,25]")
    } else if x >= 25.0 && x <= 50.0 {
        println!("Intervalo (25,50]")
    } else if x >= 50.0 && x <= 75.0 {
        println!("Intervalo (50,75]")
    } else if x >= 75.0 && x <= 100.0 {
        println!("Intervalo (75,100]")
    } else {
        println!("Fora de intervalo")
    }
}

fn input_line() -> f64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().parse::<f64>().unwrap_or_default();
}
