// https://www.beecrowd.com.br/judge/pt/problems/view/1008

use std::io;

fn main() {
    let (a, b, c) = input_data();
    let (x, y, z) = parse_data((a, b, c));

    println!("NUMBER = {}", x);
    println!("SALARY = U$ {:.2}", ((y as f32) * z) as f32);
}

fn parse_data((a, b, c): (String, String, String)) -> (i32, i32, f32) {
    let (x, y, z): (i32, i32, f32) = (
        a.trim().parse::<i32>().unwrap_or_default(),
        b.trim().parse::<i32>().unwrap_or_default(),
        c.trim().parse::<f32>().unwrap_or_default(),
    );

    return (x, y, z);
}

fn input_data() -> (String, String, String) {
    let (mut input_a, mut input_b, mut input_c): (String, String, String) =
        (String::new(), String::new(), String::new());

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();
    io::stdin().read_line(&mut input_c).unwrap();

    return (input_a, input_b, input_c);
}
