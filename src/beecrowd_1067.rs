// https://www.beecrowd.com.br/judge/pt/problems/view/1067

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let value = input.trim().parse::<i64>().unwrap();
    for i in 1..value + 1 {
        if i % 2 != 0 {
            println!("{}", i);
        }
    }
}
