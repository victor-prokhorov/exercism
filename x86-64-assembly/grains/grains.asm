section .text

global square
square:
    cmp rdi, 1
    jl .error
    cmp rdi, 64
    jg .error
    mov rax, 1
    mov rcx, rdi
    dec cl
    shl rax, cl         ; it have to be in `cl` but it cannot be omited...
    ret

.error:
    xor rax, rax
    ret

global total
total:
    xor r10, r10
    mov rcx, 64
.accumulate:
    mov rdi, rcx
    push rcx
    call square
    pop rcx
    add r10, rax
    loop .accumulate    ; TODO: i learned `LOOPcc` so i wanted to apply this but
                        ; i think the fact i trash `rcx` in `square`
                        ; make this quit clumsy
    mov rax, r10
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
