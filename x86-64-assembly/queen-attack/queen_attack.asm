section .text

global can_create
can_create:
	xor	rax, rax
	cmp	rdi, 8
	jae	.quit
	cmp	rsi, 8
	jae	.quit
	mov	rax, 1
.quit:
	ret

; memo: row - column = diagonal
;	row + column = anti-diagonal
global can_attack
can_attack:
	xor	r8, r8
	xor	r9, r9
	mov	rax, 1		; quit if we found that something is matching
; same row or same column
	cmp	rdi, rdx
	sete	r8b
	cmp	rsi, rcx
	sete	r9b
	or	r8, r9
	jnz	.quit
; same diagonal
	mov	r10, rdi
	sub	r10, rsi
	mov	r11, rdx
	sub	r11, rcx
	cmp	r10, r11
	je	.quit
; same anti-diagonal
	mov	r10, rdi
	add	r10, rsi
	mov	r11, rdx
	add	r11, rcx
	cmp	r10, r11
	je	.quit
; otherwise
	xor	rax, rax
.quit:
	ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
