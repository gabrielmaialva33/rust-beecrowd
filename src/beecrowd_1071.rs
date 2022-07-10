// https://www.beecrowd.com.br/judge/pt/problems/view/1071

use std::convert::TryFrom;

fn main() {
    let mut sun: Vec<i64> = Vec::new();
    let (min, max) = input_line();

    for i in min + 1..max {
        if i % 2 != 0 {
            sun.push(i);
        }
    }
    println!("{}", sun.iter().sum::<i64>());
}

fn input_line() -> (i64, i64) {
    let (mut a, mut b) = (String::new(), String::new());
    std::io::stdin().read_line(&mut a).ok();
    std::io::stdin().read_line(&mut b).ok();
    let a = a.trim().parse::<i64>().unwrap();
    let b = b.trim().parse::<i64>().unwrap();
    let mut vec: Vec<i64> = vec![a, b];
    vec.sort();
    let [min, max]: [i64; 2] = <[i64; 2]>::try_from(vec).unwrap();
    return (min, max);
}
