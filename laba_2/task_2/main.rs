use std::io::{self, Write};

fn roman_to_int(s: &str) -> i32 {
    let values = [
        ('I', 1), ('V', 5), ('X', 10), ('L', 50),
        ('C', 100), ('D', 500), ('M', 1000),
    ];

    let mut result = 0;
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() {
        // Ищем значение текущего символа в таблице
        let current = values.iter()
            .find(|(c, _)| *c == chars[i])
            .map(|(_, v)| *v)
            .unwrap_or(0);

        // Смотрим на следующий символ, если он есть
        let next = if i + 1 < chars.len() {
            values.iter()
                .find(|(c, _)| *c == chars[i + 1])
                .map(|(_, v)| *v)
                .unwrap_or(0)
        } else {
            0
        };

        if current < next {
            result -= current;
        } else {
            result += current;
        }
    }
    result
}

fn main() {
    let roman = loop {
        print!("Введите римское число: ");
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        let input = input.trim().to_uppercase();
        if input.is_empty() {
            continue;
        }
        if input.chars().all(|c| "IVXLCDM".contains(c)) {
            break input;
        }
        println!("Ошибка! Только римские цифры (I,V,X,L,C,D,M). Повтор: ");
    };

    let result = roman_to_int(&roman);
    println!("Результат: {}", result);
}