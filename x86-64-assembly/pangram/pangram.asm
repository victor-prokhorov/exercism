section .text

; ; O(nn)
global is_pangram
is_pangram:
    xor rax, rax
    mov rax, 'A' - 1
; a..=z
.loop:
    inc rax
    cmp al, 'Z'
    jg .found_all
    mov rsi, rdi
    xor rdx, rdx                ; found letter in string flag
; rsi[0]..=rsi[rsi.len() - 1]
.inner_loop:
    cmp byte [rsi], 0
    jz .check_found
    cmp al, byte [rsi]
    je .letter_found
    mov cl, byte [rsi]
    sub cl, 0x20
    cmp al, cl
    je .letter_found
    inc rsi
    jmp .inner_loop
.letter_found:
    mov rdx, 1
.check_found:
    test rdx, rdx
    jnz .loop
    jmp .not_found
.found_all:
    mov rax, 1
    ret
.not_found:
    xor rax, rax
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
