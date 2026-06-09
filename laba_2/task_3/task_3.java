import java.util.Scanner;

public class Main {
    
    public static int sumOfDigits(int num) {
        int sum = 0;
        while (num > 0) {
            sum += num % 10;
            num /= 10;
        }
        return sum;
    }
   
    public static int safeInput(Scanner scanner, String message) {
        while (true) {
            System.out.print(message);
            String line = scanner.nextLine();
            
            try {
                return Integer.parseInt(line.trim());
            } catch (NumberFormatException e) {
                System.out.println("Ошибка! Нужно ввести ЧИСЛО. Попробуйте ещё раз!");
            }
        }
    }
    
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        
        // Ввод N с проверкой на положительное число
        int N;
        do {
            N = safeInput(scanner, "Введите N натуральных чисел: ");
            if (N <= 0) {
                System.out.println("Ошибка! N должно быть больше 0. Попробуйте ещё раз!");
            }
        } while (N <= 0);
        
        int count = 0;
        
        for (int i = 0; i < N; i++) {
            int num = safeInput(scanner, "Введите число " + (i + 1) + ": ");
            if (sumOfDigits(num) > 10) {
                count++;
            }
        }
        
        System.out.println("Количество чисел сумма которых больше 10 равна " + count);
        scanner.close();
    }
}