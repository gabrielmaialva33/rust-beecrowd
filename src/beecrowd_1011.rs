use std::io;

fn main() {
    let r = input_line();
    println!("VOLUME = {:.3}", ((4.0 / 3.0) * 3.14159 * r.powi(3)))
}

fn input_line() -> f64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap_or_default();

    let r: f64 = line.trim().parse::<f64>().unwrap_or_default();

    return r;
}
