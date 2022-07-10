// https://www.beecrowd.com.br/judge/pt/problems/view/1051

fn main() {
    let value = input_line();
    if value >= 0.0 && value <= 2000.0 {
        println!("Isento")
    } else if value >= 2000.01 && value <= 3000.0 {
        println!("R$ {:.2}", (value - 2000.0) * 0.08)
    } else if value >= 3000.01 && value <= 4500.0 {
        println!("R$ {:.2}", (1000.0 * 0.08) + (value - 3000.0) * 0.18)
    } else if value > 4500.0 {
        println!(
            "R$ {:.2}",
            (1000.0 * 0.08) + (1500.0 * 0.18) + (value - 4500.0) * 0.28
        )
    } else {
        println!("Erro")
    }
}

fn input_line() -> f64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let value = input.trim().parse::<f64>().unwrap();
    return value;
}
