section .text

global square_of_sum
square_of_sum:
    xor rax, rax
    mov rcx, rdi
sum:
    add rax, rdi
    dec rdi
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
    call square_of_sum
    pop rdi
    push rax
    call sum_of_squares
    pop rdi
    sub rax, rdi
    ; https://stackoverflow.com/a/11927940
    mov ebx, eax
    neg eax
    cmovl eax, ebx
    ret

exit:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
