section .text
bits 32
global start
extern _start

start:
    ; Настраиваем стек
    mov esp, stack_top
    
    ; Вызываем нашу функцию _start из Rust
    call _start
    
    ; Если мы вернулись (что не должно произойти), зацикливаемся
    jmp $

section .bss
align 16
stack_bottom:
    resb 4096 ; 4 КБ для стека
stack_top: