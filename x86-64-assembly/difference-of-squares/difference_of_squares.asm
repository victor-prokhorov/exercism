section .text

global square_of_sum
square_of_sum:
    xor rax, rax
sum:
    cmp rdi, 0
    je square
    add rax, rdi
    dec rdi
    jmp sum
square:
    imul rax, rax
    ret

global sum_of_squares
sum_of_squares:
    xor rax, rax
    xor rsi, rsi
squares:
    cmp rdi, 0
    je exit
    mov rsi, rdi
    dec rdi
    imul rsi, rsi
    add rax, rsi
    jmp squares

global difference_of_squares
difference_of_squares:
    push rdi
    call square_of_sum
    pop rdi
    push rax
    call sum_of_squares
    pop rdi
    sub rax, rdi
    test rax, rax
    jns exit
    neg rax
    ret

exit:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
