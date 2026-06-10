use std::io::{self, Write};

fn gr(x: f64) -> f64 {
    if x < -2.0 {
        -2.0 * x - 5.0
    } else if x >= -2.0 && x < 0.0 {
        -1.0 - (1.0 - (x + 1.0) * (x + 1.0)).sqrt()
    } else if x >= 0.0 && x < 2.0 {
        2.0 - (4.0 - x * x).sqrt()
    } else if x >= 2.0 && x < 4.0 {
        -1.0
    } else {
        0.5 * x - 3.0
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter start, end, step: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");

    let mut tokens = input.split_whitespace();
    let x_start: f64 = tokens.next().expect("Не указано начальное значение").parse().unwrap();
    let x_end: f64   = tokens.next().expect("Не указано конечное значение").parse().unwrap();
    let dx: f64      = tokens.next().expect("Не указан шаг").parse().unwrap();

    println!("|     x      |    f(x)    |");
    println!("|------------|------------|");

    let mut x = x_start;
    while x <= x_end + 0.001 {
        println!("| {:>10.2} | {:>10.2} |", x, gr(x));
        x += dx;
    }
}