section .text

global leap_year
leap_year:
	xor rax, rax		; returned boolean
	mov rax, rdi		; move year from argument to dividend
	xor rdx, rdx		; high part of dividend
	mov rcx, 4		; divisor
	div rcx			; quotient `rax = rax / rcx`
				; remainder `rdx = rax % rcx`
	test rdx, rdx
	jne not_leap
				; naturally at this point we already know `rdx` is zero
	mov rcx, 100		; divisor
	mov rax, rdi		; `rax` were trashed, `rdi` still holds year	
	div rcx
	test rdx, rdx
	jne leap
	mov rcx, 400		; here we go again
	mov rax, rdi
	div rcx
	test rdx, rdx
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
