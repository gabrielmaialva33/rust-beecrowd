// https://www.beecrowd.com.br/judge/pt/problems/view/1074

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<i64>().unwrap();
    for _ in 1..n + 1 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let value = input.trim().parse::<i64>().unwrap();
        if value == 0 {
            println!("NULL");
        } else if value % 2 == 0 {
            if value > 0 {
                println!("EVEN POSITIVE");
            } else {
                println!("EVEN NEGATIVE");
            }
        } else {
            if value > 0 {
                println!("ODD POSITIVE");
            } else {
                println!("ODD NEGATIVE");
            }
        }
    }
}