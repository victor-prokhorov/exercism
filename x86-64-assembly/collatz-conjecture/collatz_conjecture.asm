section .text
global steps
steps:
    xor r10, r10        ; steps count
    test edi, edi       ; compare the lower part of the first argument rdi
    jle error           ; if less then or equal to 0
loop:
    cmp rdi, 1          ; did we reached the end?
    jl error            ; we are expected to get to 1 without going bellow
    je done             ; go to return procedure
    inc r10             ; increment steps count
    mov rax, rdi        ; move first argument into accumulator register
    and al, 1           ; check if LSB is 1
    cmp al, 0           
    je is_even
    jmp is_odd
is_even:
    shr rdi, 1
    jmp loop
is_odd:
    mov rax, 3
    mul rdi
    inc rax
    mov rdi, rax
    jmp loop
error:
    mov rax, -1         ; save error code
    ret
done:
    mov rax, r10        ; save steps count to return register
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
