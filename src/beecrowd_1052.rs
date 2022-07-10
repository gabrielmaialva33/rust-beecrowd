// https://www.beecrowd.com.br/judge/pt/problems/view/1051

fn main() {
    let x = input_line();
    match x {
        1 => println!("January"),
        2 => println!("February"),
        3 => println!("March"),
        4 => println!("April"),
        5 => println!("May"),
        6 => println!("June"),
        7 => println!("July"),
        8 => println!("August"),
        9 => println!("September"),
        10 => println!("October"),
        11 => println!("November"),
        12 => println!("December"),
        _ => {}
    }
}

fn input_line() -> i64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    return input.trim().parse::<i64>().unwrap();
}
