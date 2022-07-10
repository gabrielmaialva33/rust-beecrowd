// https://www.beecrowd.com.br/judge/pt/problems/view/1048

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let value = input.trim().parse::<f64>().unwrap();
    if value >= 0.0 && value <= 400.0 {
        println!("Novo salario: {:.2}", value * 1.15);
        println!("Reajuste ganho: {:.2}", value * 0.15);
        println!("Em percentual: 15 %");
    } else if value >= 400.01 && value <= 800.0 {
        println!("Novo salario: {:.2}", value * 1.12);
        println!("Reajuste ganho: {:.2}", value * 0.12);
        println!("Em percentual: 12 %");
    } else if value >= 800.01 && value <= 1200.0 {
        println!("Novo salario: {:.2}", value * 1.10);
        println!("Reajuste ganho: {:.2}", value * 0.10);
        println!("Em percentual: 10 %");
    } else if value >= 1200.01 && value <= 2000.0 {
        println!("Novo salario: {:.2}", value * 1.07);
        println!("Reajuste ganho: {:.2}", value * 0.07);
        println!("Em percentual: 7 %");
    } else if value >= 2000.01 {
        println!("Novo salario: {:.2}", value * 1.04);
        println!("Reajuste ganho: {:.2}", value * 0.04);
        println!("Em percentual: 4 %");
    }
}
