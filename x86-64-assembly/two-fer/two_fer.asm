default rel

section .rodata

lhs: db "One for "
lhs_len equ $ - lhs             ; `$` is current location, we sub `lhs` addr, we get `len`!
rhs: db ", one for me.", 0
rhs_len equ $ - rhs
you: db "you"
you_len equ $ - you


section .text

; https://exercism.org/tracks/x86-64-assembly/exercises/two-fer/solutions/udhos
; essentially my walktrough this solution
global two_fer
; `rdi` friends name, falls back to `you` if none were passed
; `rsi` formed string
two_fer:
    mov rax, rdi                ; we need to save it because we will use `src index` and `dst index` with `movsb`
    mov rdi, rsi                ; it's first argument but now we need it as destination
    lea rsi, [lhs]              ; we will write from `lhs` ptr
    mov rcx, lhs_len            ; counter of bytes we are about to write
    rep movsb                   ; https://web.itu.edu.tr/kesgin/mul06/intel/instr/rep.html
                                ; `rep` repeats execution while `cx != 0`
                                ; `movsb` copy from `rsi` (source index)
                                ;              into `rdi` (destination index)
                                ; https://faydoc.tripod.com/cpu/movsb.htm
    cmp rax, 0
    jne .write_non_null_name
    lea rsi, [you]
    mov rcx, you_len
    rep movsb
    jmp .write_rhs
.write_non_null_name:
    mov rsi, rax                ; `rax` have the saved friends name from `rdi`
    ; cmp byte [rsi], 0   ; ? do we really need this, i think we already checked that it is not null
.write_until_null_termination:
    ; `rdi` contains is still/already at the good position
    cmp byte [rsi], 0
    je .write_rhs
    movsb ; to `rdi` from `rsi` 1 byte
    jmp .write_until_null_termination
.write_rhs:
    lea rsi, [rhs]
    mov rcx, rhs_len
    rep movsb
    ret






%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
