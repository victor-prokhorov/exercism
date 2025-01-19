default rel
section .bss
buffer resb 128
section .text
global reverse
reverse:
	mov rax, rdi
	lea rsi, [buffer]
.copy_to_buffer:
	mov dl, byte [rax]
	cmp dl, 0
	jz .setup_reversing
	mov byte [rsi], dl
	inc rax
	inc rsi
	jmp .copy_to_buffer
.setup_reversing:
	dec rsi
	mov rax, rdi
.copy_from_buffer:
	mov dl, byte [rsi]
	mov byte [rax], dl
	dec rsi
	inc rax
	mov dl, byte [rax]
	cmp dl, 0
	jnz .copy_from_buffer
.exit:
	ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
