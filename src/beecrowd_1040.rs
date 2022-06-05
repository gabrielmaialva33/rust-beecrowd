// https://www.beecrowd.com.br/judge/pt/problems/view/1040

use std::convert::TryFrom;
use std::io;

fn main() {
    let (n1, n2, n3, n4) = input_line();
    let media = ((n1 * 2.0) + (n2 * 3.0) + (n3 * 4.0) + (n4 * 1.0)) / 10.0;
    println!("Media: {:.1}", media);
    if media < 5.0 {
        println!("Aluno reprovado.");
    } else if media >= 5.0 && media <= 6.9 {
        println!("Aluno em exame.");

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let n5 = line.trim().parse::<f64>().unwrap_or_default();
        println!("Nota do exame: {:.1}", n5);
        let new_media = (media + n5) / 2.0;
        if new_media >= 5.0 {
            println!("Aluno aprovado.");
        } else {
            println!("Aluno reprovado.");
        };
        println!("Media final: {:.1}", new_media);
    } else {
        println!("Aluno aprovado.");
    }
}

fn input_line() -> (f64, f64, f64, f64) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let [w, x, y, z]: [String; 4] = <[String; 4]>::try_from(
        line.split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let (n1, n2, n3, n4) = (
        w.trim().parse::<f64>().unwrap_or_default(),
        x.trim().parse::<f64>().unwrap_or_default(),
        y.trim().parse::<f64>().unwrap_or_default(),
        z.trim().parse::<f64>().unwrap_or_default(),
    );

    return (n1, n2, n3, n4);
}
