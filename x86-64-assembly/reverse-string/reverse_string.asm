section .text

; `rdi` pointer to string to reverse, we traverse it left to right, only `dil` non zero
; `[rsi - 1]` will point to the last character, will be moved right to left
; `rax` or `al` more precisely will hold value to swap, left hand side
; `rdx` will hold temporary character of 1 byte, thus only `dl` will be used, right hand side
global reverse
reverse:
        mov     rsi, rdi
find_last_position:
        cmp     byte [rsi], 0
        jz      swap
        inc     rsi
        jmp     find_last_position
swap:
        cmp     rdi, rsi - 1
        jge     exit
        mov     al, byte [rdi]
        mov     dl, byte [rsi - 1]
        mov     byte [rdi], dl
        mov     byte [rsi - 1], al
        inc     rdi
        dec     rsi
        jmp     swap
exit:
        ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
