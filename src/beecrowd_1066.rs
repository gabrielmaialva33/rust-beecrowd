// https://www.beecrowd.com.br/judge/pt/problems/view/1066

fn main() {
    let (mut even, mut odd, mut positive, mut negative) = (0, 0, 0, 0);
    for _ in 0..5 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let value = input.trim().parse::<i64>().unwrap();
        if value % 2 == 0 {
            even += 1;
        }
        if value % 2 != 0 {
            odd += 1;
        }
        if value > 0 {
            positive += 1;
        }
        if value < 0 {
            negative += 1;
        }
    }

    println!("{} valor(es) par(es)", even);
    println!("{} valor(es) impar(es)", odd);
    println!("{} valor(es) positivo(s)", positive);
    println!("{} valor(es) negativo(s)", negative);
}
