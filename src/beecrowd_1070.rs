// https://www.beecrowd.com.br/judge/pt/problems/view/1070

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let value = input.trim().parse::<i64>().unwrap();
    for i in value..value + 12 {
        if i % 2 != 0 {
            println!("{}", i);
        }
    }
}
