section .text

; https://exercism.org/tracks/x86-64-assembly/exercises/hamming/solutions/paiv
global distance
distance:
    xor rax, rax
    xor rcx, rcx
    xor rdx, rdx
.loop:
    mov r8b, byte [rdi + rcx]
    mov r9b, byte [rsi + rcx]
    inc rcx
    cmp r8b, r9b
    setne dl
    add rax, rdx
    or r8b, r9b
    ; 0, 0 -> 0 -> both have same lengths, clean exit
    ; 1, 0 | 0, 1 | 1, 1 -> 1 -> at least one side is non null
    je .exit
    test r8b, r9b
    ; at this point, we already know that it is not `null, null`
    ; 0, 1 -> 0 -> if on one of the sides we have 0 we get 0
    ; i.e. if we get `1` we should have `1` on both sides
    ;
    ; thus if we get `0` we have null termination on one side
    ; while having a character on other side, exit with error code `-1`
    jne .loop
    mov rax, -1
.exit:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
