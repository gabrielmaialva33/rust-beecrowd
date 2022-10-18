// https://www.beecrowd.com.br/judge/pt/problems/view/1072

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n = n.trim().parse::<i64>().unwrap();
    let mut vec: Vec<i64> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let value = input.trim().parse::<i64>().unwrap();
        vec.push(value);
    }

    println!("{} in", vec.iter().filter(|&x| *x >= 10 && *x <= 20).count());
    println!("{} out", vec.iter().filter(|&x| *x < 10 || *x > 20).count());
}
