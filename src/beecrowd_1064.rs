// https://www.beecrowd.com.br/judge/pt/problems/view/1064

fn main() {
    let mut values: Vec<f64> = Vec::new();
    for _ in 0..6 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let value = input.trim().parse::<f64>().unwrap();
        if value > 0.0 {
            values.push(value);
        }
    }

    println!(
        "{} valores positivos\n{:.1}",
        values.len(),
        values.iter().sum::<f64>() / values.len() as f64
    );
}
