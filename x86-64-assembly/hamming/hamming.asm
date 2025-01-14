section .text

global distance
distance:
    xor rax, rax
.loop:
    mov r8b, byte [rdi]
    mov r9b, byte [rsi]
    inc rdi
    inc rsi
    cmp r8b, 0
    sete r10b
    cmp r9b, 0
    sete r11b
    ; cmp r10b, r11b - does not work for the cases where it's false false
    add r10b, r11b
    cmp r10b, 2
    je .eq
    cmp r10b, 1
    je .neq
    cmp r8b, r9b
    je .loop
    ; mov rax, 666
    inc rax
    jmp .loop
.neq:
    mov rax, -1
    ret
.eq:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
