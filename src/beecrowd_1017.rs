// https://www.beecrowd.com.br/judge/pt/problems/view/1017

use std::io;

fn main() {
    let (mut t, mut v): (i32, i32) = (0, 0);
    for i in 0..2 {
        match i {
            0 => t = input_line(),
            1 => v = input_line(),
            _ => {}
        }
    }

    println!("{:.3}", (t as f64 * v as f64) / 12.0)
}

fn input_line() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let i: i32 = line.trim().parse::<i32>().unwrap_or_default();
    return i;
}
