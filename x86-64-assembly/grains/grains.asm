section .text

global square
square:
    mov rcx, rdi
    dec rcx
    ; lea rcx, [rdi - 1] - https://exercism.org/tracks/x86-64-assembly/exercises/grains/solutions/0x28
    ; super fun trick, looks like we get offset from null because of `[]` so we get "arithmetic value"
    ; so we end with value in `rcx` that is `rdi - 1` in one operation!
    cmp rcx, 64
    jae .error
    mov rax, 1
    shl rax, cl
    ret
    ; cmp rdi, 1
    ; jl .error
    ; cmp rdi, 64
    ; jg .error
    ; mov rax, 1
    ; mov rcx, rdi
    ; dec cl
    ; shl rax, cl         ; it have to be in `cl` but it cannot be omited...
    ; ret

.error:
    xor rax, rax
    ret

global total
total:
    ; of course... Mn = 2^n - 1
    ; it's 64bits, thus 2^64 - 1 is just `-1`
    mov rax, -1
    ret
    ;     xor r10, r10
    ;     mov rcx, 64
    ; .accumulate:
    ;     mov rdi, rcx
    ;     push rcx
    ;     call square
    ;     pop rcx
    ;     add r10, rax
    ;     loop .accumulate    ; TODO: i learned `LOOPcc` so i wanted to apply this but
    ;                         ; i think the fact i trash `rcx` in `square`
    ;                         ; make this quit clumsy
    ;     mov rax, r10
    ;     ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
