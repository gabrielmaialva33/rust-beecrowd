// https://www.beecrowd.com.br/judge/pt/problems/view/1014

use std::io;

fn main() {
    let (mut x, mut y): (i32, f64) = (0, 0.0);
    for i in 0..2 {
        let line = input_line();
        match i {
            0 => x = line.trim().parse::<i32>().unwrap_or_default(),
            1 => y = line.trim().parse::<f64>().unwrap_or_default(),
            _ => {}
        }
    }

    println!("{:.3} km/l", x as f64 / y);
}

fn input_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line;
}
