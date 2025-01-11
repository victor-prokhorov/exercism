section .text

global square_of_sum
square_of_sum:
    xor rax, rax
    mov rcx, rdi
sum:
    add rax, rcx
    loop sum
    imul rax, rax
    ret

global sum_of_squares
sum_of_squares:
    xor rax, rax
    mov rcx, rdi
squares:
    mov rdi, rcx
    imul rdi, rdi
    add rax, rdi
    loop squares
    ret

global difference_of_squares
difference_of_squares:
    push rdi
    call sum_of_squares
    pop rdi
    mov rdx, rax
    push rdi
    call square_of_sum
    pop rdi
    sub rax, rdx
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
