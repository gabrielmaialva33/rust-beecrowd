// https://www.beecrowd.com.br/judge/pt/problems/view/1060

fn main() {
    let mut count = 0;
    for _ in 0..6 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let value = input.trim().parse::<f64>().unwrap();
        if value > 0.0 {
            count += 1;
        }
    }
    println!("{} valores positivos", count);
}
