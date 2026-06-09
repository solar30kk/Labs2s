def longest_ones(s, K):
    left = 0
    zero_count = 0
    max_len = 0
    
    for right in range(len(s)):
        if s[right] == '0':
            zero_count += 1
        
        while zero_count > K:
            if s[left] == '0':
                zero_count -= 1
            left += 1
        
        max_len = max(max_len, right - left + 1)
    
    return max_len

def get_valid_binary_string():
    while True:
        s = input("Введите строку из 0 и 1: ").strip()
        if not s:
            print("Пустая строка! Попробуйте ещё раз.")
            continue
        if all(c in '01' for c in s):
            return s
        print("Вам нужно ввести строку из 0 и 1. Попробуйте еще раз!")

def main():
    s = get_valid_binary_string()
    
    while True:
        try:
            K = int(input("Введите K (сколько нулей можно заменить): "))
            break
        except ValueError:
            print("Ошибка! Нужно ввести ЧИСЛО. Попробуйте ещё раз: ")
    
    result = longest_ones(s, K)
    print(result)

if __name__ == "__main__":
    main()