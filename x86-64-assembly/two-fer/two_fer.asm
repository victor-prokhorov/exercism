default rel

section .rodata

lhs: db "One for ", 0
rhs: db ", one for me.", 0
you: db "you", 0


section .text

global two_fer
; `rdi` friends name, falls back to `you` if none were passed
; `rsi` formed string
two_fer:
    xor r10, r10        ; offset, i could move the pointer itself but i want to try this
    lea r11, [lhs]      ; &lhs
write_lhs:
    mov al, byte [r10 + r11]
    cmp al, 0
    je write_name
    mov byte [rsi], al
    inc r10
    inc rsi
    jmp write_lhs
write_name:
    cmp rdi, 0
    je prepare_write_you
    ; i'm about to copy the whole loop block, the only thing that changes is src and dst
    ; feels like a good macro use case
    xor r10, r10
    lea r11, [rdi] ; could use rdi at this point instead of r11
write_loop:
    mov al, byte [r10 + r11]
    cmp al, 0
    je prepare_rhs
    mov byte [rsi], al
    inc r10
    inc rsi
    jmp write_you
prepare_write_you:
    xor r10, r10
    lea r11, [you]
write_you:
    mov al, byte [r10 + r11]
    cmp al, 0
    je prepare_rhs
    mov byte [rsi], al
    inc r10
    inc rsi
    jmp write_you
prepare_rhs:
    xor r10, r10
    lea r11, [rhs]
write_rhs:
    mov al, byte [r10 + r11]
    cmp al, 0
    je exit
    mov byte [rsi], al
    inc r10
    inc rsi
    jmp write_rhs
exit:
    mov byte [rsi], 0 ; if name is not null we need to add null termination
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
