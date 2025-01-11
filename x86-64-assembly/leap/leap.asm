; https://exercism.org/tracks/x86-64-assembly/exercises/leap/solutions/Silva97

section .text

%macro divisible 2
    mov rax, %1
    xor rdx, rdx
    mov rcx, %2
    div rcx
    test rdx, rdx
%endmacro

global leap_year
leap_year:
    divisible rdi, 4
    jne not_leap
    divisible rdi, 100
    jne leap
    divisible rdi, 400
    jne not_leap
leap:
    mov rax, 1
    ret 
not_leap:
    xor rax, rax
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
