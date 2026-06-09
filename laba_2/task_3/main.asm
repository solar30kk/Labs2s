section .bss
    N           resd 1
    count       resd 1
    current_num resd 1
    digit_sum   resd 1
    i           resd 1          ; Добавили счетчик цикла

section .data
    fmt_in      db "%d", 0
    fmt_out     db "%d", 10, 0

section .text
    global main
    extern printf, scanf

main:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    
    ; Чтение N
    lea rdi, [fmt_in]
    lea rsi, [N]
    xor rax, rax
    call scanf
    
    ; Инициализация
    mov dword [count], 0
    mov dword [i], 0
    
.loop_start:
    ; Проверка: если i >= N, выходим
    mov eax, [i]
    cmp eax, [N]
    jge .loop_end
    
    ; Чтение очередного числа
    lea rdi, [fmt_in]
    lea rsi, [current_num]
    xor rax, rax
    call scanf
    
    ; Вычисление суммы цифр
    mov eax, [current_num]
    mov dword [digit_sum], 0
    
.calc_digit_sum:
    cmp eax, 0
    je .check_sum
    
    ; digit = num % 10
    mov ebx, 10
    xor edx, edx
    div ebx             ; eax = num/10, edx = num%10
    
    ; sum += digit
    add [digit_sum], edx
    
    ; num = num / 10 (eax уже содержит результат деления)
    jmp .calc_digit_sum
    
.check_sum:
    ; if (digit_sum > 10) count++
    mov eax, [digit_sum]
    cmp eax, 10
    jle .skip_increment
    mov eax, [count]
    inc eax
    mov [count], eax
    
.skip_increment:
    ; i++
    mov eax, [i]
    inc eax
    mov [i], eax
    jmp .loop_start
    
.loop_end:
    ; Вывод результата (count)
    mov eax, [count]
    lea rdi, [fmt_out]
    mov rsi, rax        ; Передаем count как второй аргумент
    xor rax, rax
    call printf
    
    xor eax, eax
    leave
    ret