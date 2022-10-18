// https://www.beecrowd.com.br/judge/pt/problems/view/1073

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<i64>().unwrap();
    for i in 1..n+1 {
        if i % 2 == 0 {
            println!("{}^2 = {}",i, i.pow(2));
        }
    }
}