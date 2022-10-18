// https://www.beecrowd.com.br/judge/pt/problems/view/1075

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<i64>().unwrap();
    for i in 1..10000 {
        if i % n == 2 {
            println!("{}", i);
        }
    }
}