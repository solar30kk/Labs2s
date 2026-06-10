use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Считываем количество чисел и максимальное количество для взятия
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.trim().split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let m: i32 = parts.next().unwrap().parse().unwrap();
    
    // Считываем массив чисел
    let a: Vec<i32> = lines.next().unwrap().unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    // Вычисляем префиксные суммы для быстрого подсчёта сумм на отрезках
    let mut prefix = vec![0; (n + 1) as usize];
    for i in 0..n as usize {
        prefix[i + 1] = prefix[i] + a[i];
    }
    
    // Счета игроков и позиция в массиве
    let mut pavel_score = 0;
    let mut vika_score = 0;
    let mut pos = 0;
    
    // Сколько чисел взял каждый игрок в свой предыдущий ход
    // -1 означает, что хода ещё не было
    let mut last_pavel = -1;
    let mut last_vika = -1;
    
    // Чей ход: 0 - Павел, 1 - Вика
    let mut turn = 0;
    
    // Играем пока не кончатся числа
    while pos < n {
        // Максимальное количество чисел, которое можно взять
        let max_take = std::cmp::min(m, n - pos);
        
        // Запрещённое количество (сколько нельзя брать)
        let forbidden = if turn == 0 {
            last_pavel  // Павел не может взять столько, сколько взял в прошлый раз
        } else {
            last_vika   // Вика не может взять столько, сколько взяла в прошлый раз
        };
        
        // Ищем лучший ход
        let mut best_sum = -1_000_000_000;  // Очень маленькое число
        let mut best_take = 1;
        
        // Пробуем взять от 1 до max_take чисел
        for take in 1..=max_take {
            // Пропускаем запрещённое количество
            if take == forbidden {
                continue;
            }
            
            // Считаем сумму take чисел начиная с позиции pos
            let current_sum = prefix[(pos + take) as usize] - prefix[pos as usize];
            
            // Если сумма больше, или сумма такая же, но берём меньше чисел
            if current_sum > best_sum || (current_sum == best_sum && take < best_take) {
                best_sum = current_sum;
                best_take = take;
            }
        }
        
        // Добавляем сумму к счёту текущего игрока
        if turn == 0 {
            pavel_score += best_sum;
            last_pavel = best_take;  // Запоминаем, сколько взял Павел
        } else {
            vika_score += best_sum;
            last_vika = best_take;   // Запоминаем, сколько взяла Вика
        }
        
        // Переходим к следующей позиции
        pos += best_take;
        
        // Меняем игрока
        turn = 1 - turn;
    }
    
    // Определяем победителя
    if pavel_score > vika_score {
        println!("1");
    } else {
        println!("0");
    }
}