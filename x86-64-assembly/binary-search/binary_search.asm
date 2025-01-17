section .text

; extern int find(int *array, int size, int value);
; (rdi, rsi, rdx) -> rax
; div src need `mov rdx,0`
; `rax=rax/rcx` and `rdx=rax%rcx`
; TEST_ASSERT_EQUAL_INT(0, find(array, ARRAY_SIZE(array), 6));
; rax is the index
; int array[] = {1, 3, 4, 6, 8, 9, 11};
; TEST_ASSERT_EQUAL_INT(-1, find(array, ARRAY_SIZE(array), 7));
; or -1 flag if not found
; rax is the index no surprise
; r8 lo
; r9 mi
; r10 hi
; r11 value we are looking because we need rdx for divisions

section .text

global find
find:
        mov     r11, rdx                        ; save target value
        xor     rdx, rdx                        ; clear for division
        xor     r8, r8                          ; lower bound
        mov     r10, rsi                        ; higher bound
.loop:
        cmp     r8, r10                         ; until search space is valid
        jge     .not_found
        mov     rax, r10                        ; accumulate next candidate index
        sub     rax, r8                         ; subtract lower bound
        shr     rax, 1                          ; take middle element of that range
        add     rax, r8                         ; offset by lower bound to get middle index
        mov     r9, rax                         ; save middle index
        mov     eax, dword [rdi + 4 * r9]       ; load value at middle index
        cmp     rax, r11                        ; compare middle value with target
        jz      .found
        jg      .move_left
        mov     r8, r9
        inc     r8
        jmp     .loop
.move_left:
        mov     r10, r9
        jmp     .loop
.found:
        mov     rax, r9
        ret
.not_found:
        mov     rax, -1
        ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
