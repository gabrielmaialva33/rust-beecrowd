// https://www.beecrowd.com.br/judge/pt/problems/view/1049

fn main() {
    let (mut input_a, mut input_b, mut input_c) = (String::new(), String::new(), String::new());
    std::io::stdin().read_line(&mut input_a).unwrap();
    std::io::stdin().read_line(&mut input_b).unwrap();
    std::io::stdin().read_line(&mut input_c).unwrap();

    let a = input_a.trim();
    let b = input_b.trim();
    let c = input_c.trim();

    match a {
        "vertebrado" => match b {
            "ave" => match c {
                "carnivoro" => println!("aguia"),
                "onivoro" => println!("pomba"),
                _ => println!("erro"),
            },
            "mamifero" => match c {
                "onivoro" => println!("homem"),
                "herbivoro" => println!("vaca"),
                _ => println!("erro"),
            },
            _ => println!("erro"),
        },
        "invertebrado" => match b {
            "inseto" => match c {
                "hematofago" => println!("pulga"),
                "herbivoro" => println!("lagarta"),
                _ => println!("erro"),
            },
            "anelideo" => match c {
                "hematofago" => println!("sanguessuga"),
                "onivoro" => println!("minhoca"),
                _ => println!("erro"),
            },
            _ => println!("erro"),
        },
        _ => println!("erro"),
    }
}
