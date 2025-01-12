section .text
global square_root
square_root:
    mov rax, 1
increment:
    mov rsi, rax
    imul rsi, rsi
    cmp rsi, rdi
    je exit
    inc rax
    jmp increment
exit:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
