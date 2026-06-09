use std::io::{self, Write};

fn longest_ones(s: &str, k: i32) -> i32 {
    let s_bytes = s.as_bytes();
    let mut left = 0;
    let mut zero_count = 0;
    let mut max_len = 0;
    let k = k as usize;

    for right in 0..s_bytes.len() {
        if s_bytes[right] == b'0' {
            zero_count += 1;
        }

        while zero_count > k {
            if s_bytes[left] == b'0' {
                zero_count -= 1;
            }
            left += 1;
        }
        max_len = max_len.max(right - left + 1);
    }
    max_len as i32
}

fn get_valid_binary_string() -> String {
    loop {
        print!("Введите строку из 0 и 1: ");
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();

        if s.is_empty() {
            println!("Пустая строка! Попробуйте ещё раз.");
            continue;
        }

        if s.chars().all(|c| c == '0' || c == '1') {
            return s.to_string();
        } else {
            println!("Вам нужно ввести строку из 0 и 1. Попробуйте еще раз!");
        }
    }
}

fn get_valid_integer(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(k) => return k,
            Err(_) => {
                println!("Ошибка! Нужно ввести ЧИСЛО. Попробуйте ещё раз: ");
            }
        }
    }
}

fn main() {
    let s = get_valid_binary_string();
    let k = get_valid_integer("Введите K (сколько нулей можно заменить): ");
    println!("Результат: {}", longest_ones(&s, k));
}