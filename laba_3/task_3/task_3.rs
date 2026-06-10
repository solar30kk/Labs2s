//use std::cmp::Ordering;

fn nod(mut x: u64, mut y: u64) -> u64 {
    // НОД (алгоритм Евклида)
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn main() {
    let mut input = String::new();
    
    // Читаем power и base
    std::io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let power: u64 = parts.next().unwrap().parse().unwrap();
    let base: u64 = parts.next().unwrap().parse().unwrap();

    if base <= 1 {
        // если ряд расходится
        println!("infinity");
        return;
    }

    let mut sum: f64 = 0.0; // накопленная сумма ряда
    
    for n in 1..=20000u64 {
        // вычисляем сумму ряда
        
        let mut numerator: f64 = 1.0;
        for _ in 0..power {
            // Вычисляем n^power
            numerator *= n as f64;
        }

        let mut denominator: f64 = 1.0;
        for _ in 0..n {
            // Вычисляем base^n
            denominator *= base as f64;
        }

        let term = numerator / denominator;
        sum += term;

        // Если слагаемое стало очень маленьким, останавливаемся
        if term < 1e-18 {
            break;
        }
    }

    let mut best_num: u64 = 1;
    let mut best_den: u64 = 1;
    let mut best_diff: f64 = 1e20;

    // Перебираем знаменатели, чтобы найти лучшую дробь
    for den in 1..=2000000u64 {
        // Находим ближайший числитель
        let num = (sum * den as f64 + 0.5) as u64;

        // Проверяем насколько близко
        let diff = ((num as f64 / den as f64) - sum).abs();

        if diff < best_diff {
            best_diff = diff;
            best_num = num;
            best_den = den;
            
            // Если нашли очень точное совпадение - можно остановиться
            if diff < 1e-15 {
                break;
            }
        }
    }

    // Сокращаем дробь
    let common = nod(best_num, best_den);
    println!("{}/{}", best_num / common, best_den / common);
}