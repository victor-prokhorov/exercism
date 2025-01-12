section .text

; `extern void to_rna(const char *strand, char *buffer);`
; `rdi` `strand` read from this pointer
; `rsi` `buffer` write to this pointer
global to_rna
to_rna:
        xor     rax, rax
translate:
        cmp     byte [rdi], 0
        jz      terminate
        cmp     byte [rdi], 'G'
        je      g_to_c
        cmp     byte [rdi], 'C'
        je      c_to_g
        cmp     byte [rdi], 'T'
        je      t_to_a
        cmp     byte [rdi], 'A'
        je      a_to_u
g_to_c:        
        mov     byte [rsi], 'C'
        jmp     offset_and_continue
c_to_g:        
        mov     byte [rsi], 'G'
        jmp     offset_and_continue
t_to_a:        
        mov     byte [rsi], 'A'
        jmp     offset_and_continue
a_to_u:        
        mov     byte [rsi], 'U'
offset_and_continue:
        inc     rdi
        inc     rsi
        jmp     translate
terminate:
        mov     byte [rsi], 0
        ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
