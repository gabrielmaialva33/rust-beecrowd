// https://www.beecrowd.com.br/judge/pt/problems/view/1007

use std::io;

fn main() {
    let (mut input_a, mut input_b, mut input_c, mut input_d) =
        (String::new(), String::new(), String::new(), String::new());

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    io::stdin().read_line(&mut input_c).unwrap();
    io::stdin().read_line(&mut input_d).unwrap();

    let (a, b, c, d): (i32, i32, i32, i32) = (
        input_a.trim().parse::<i32>().unwrap_or_default(),
        input_b.trim().parse::<i32>().unwrap_or_default(),
        input_c.trim().parse::<i32>().unwrap_or_default(),
        input_d.trim().parse::<i32>().unwrap_or_default(),
    );

    println!("DIFERENCA = {}", ((a * b) - (c * d)))
}
