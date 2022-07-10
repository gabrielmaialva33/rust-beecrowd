// https://www.beecrowd.com.br/judge/pt/problems/view/1050

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let x = input.trim();
    match x {
        "61" => println!("Brasilia"),
        "71" => println!("Salvador"),
        "11" => println!("Sao Paulo"),
        "21" => println!("Rio de Janeiro"),
        "32" => println!("Juiz de Fora"),
        "19" => println!("Campinas"),
        "27" => println!("Vitoria"),
        "31" => println!("Belo Horizonte"),
        _ => println!("DDD nao cadastrado"),
    }
}
