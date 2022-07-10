// https://www.beecrowd.com.br/judge/pt/problems/view/1061

use std::convert::TryFrom;

fn main() {
    let (mut init_event, mut end_event): (i32, i32) = (0, 0);
    for i in 0..2 {
        match i {
            0 => init_event = input_datetime(),
            1 => end_event = input_datetime(),
            _ => {}
        }
    }

    println!("{} dia(s)", ((end_event - init_event).abs() / 86400));
    println!(
        "{} hora(s)",
        ((end_event - init_event).abs() % 86400) / 3600
    );
    println!(
        "{} minuto(s)",
        ((((end_event - init_event).abs() % 86400) % 3600) / 60)
    );
    println!(
        "{} segundo(s)",
        ((((end_event - init_event).abs() % 86400) % 3600) % 60)
    );
}

fn input_datetime() -> i32 {
    let mut date = String::new();
    std::io::stdin().read_line(&mut date).ok();
    let [d]: [i32; 1] = <[i32; 1]>::try_from(
        date.splitn(1, "Dia")
            .map(|x| x.trim().replace("Dia ", "").parse::<i32>().unwrap())
            .collect::<Vec<i32>>(),
    )
    .unwrap();

    let mut time = String::new();
    std::io::stdin().read_line(&mut time).ok();
    let [h, m, s]: [i32; 3] = <[i32; 3]>::try_from(
        time.split(":")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>(),
    )
    .unwrap();

    return d * 86400 + h * 3600 + m * 60 + s;
}
