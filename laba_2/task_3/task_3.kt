import java.util.Scanner

fun sumOfDigits(num: Int): Int {
    var n = if (num < 0) -num else num  // обрабатываем отрицательные числа
    var sum = 0
    while (n > 0) {
        sum += n % 10
        n /= 10
    }
    return sum
}

fun main() {
    val scanner = Scanner(System.`in`)
    
    var N: Int
    do {
        print("Введите N натуральных чисел: ")
        while (!scanner.hasNextInt()) {
            println("Ошибка! Нужно ввести ЧИСЛО. Попробуйте ещё раз!")
            scanner.next()  // очищаем неправильный ввод
            print("Введите N натуральных чисел: ")
        }
        N = scanner.nextInt()
        if (N <= 0) {
            println("Ошибка! N должно быть больше 0. Попробуйте ещё раз!")
        }
    } while (N <= 0)
    
    var count = 0
    for (i in 1..N) {
        print("Введите число $i: ")
        while (!scanner.hasNextInt()) {
            println("Ошибка! Нужно ввести ЧИСЛО. Попробуйте ещё раз!")
            scanner.next()
            print("Введите число $i: ")
        }
        val num = scanner.nextInt()
        if (sumOfDigits(num) > 10) {
            count++
        }
    }
    
    println("Количество чисел, сумма цифр которых больше 10, равна $count")
}