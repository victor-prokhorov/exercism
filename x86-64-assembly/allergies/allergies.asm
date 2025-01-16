section .text

; extern int allergic_to(enum item item, unsigned int score);
;        rax                        rdi                 rsi
; enum item { EGGS, PEANUTS, ... };
;
; enum variant   enum value         bitmask
; ----------------------------------------------
; EGGS	  	   (1)		`2 ^ 0 = 2 ^ rdi`
; PEANUTS	   (2)	 	`2 ^ 1`
;
; TEST_ASSERT_TRUE(allergic_to(EGGS, 1));
global allergic_to
allergic_to:
	xor	rax,rax
	inc	rax
	mov	rcx,rdi
	shl	rax,cl
	test	rax,rsi
	jz	.not_allergic
	mov	rax,1
	ret
.not_allergic:
	xor	rax,rax
	ret

; extern void list(unsigned int score, struct item_list *list);
;                                 rdi                     rsi
; test_eggs_and_peanuts
; `int` or `32 bits` or `4 bytes` or `dword`
; mov dword [rsi],     2	list size
; mov dword [rsi + 4], 0	allergic to eggs
; mov dword [rsi + 8], 1 	allergic to peanuts
global list
list:
	mov	rdx,rsi				; save item list pointer because we need 2nd argument to make a call
	mov	rsi,rdi				; copy received score into passed score
	xor	rax,rax
	xor	rcx,rcx				; loop `check_score` candidate value
	dec	rcx				; score can start at 0 so we set -1 before the loop
	xor	r8,r8				; growing list size
	mov	dword[rdx],0			; must init, seg fault otherwise
.check_score:
	inc	rcx
	cmp	rcx,8
	jge	.quit	
	mov	rdi,rcx				; copy candidate value into item argument
	call	allergic_to		
	mov	r8d,dword[rdx]			; get size value
	add	r8,rax				; add to size counter if appropriate, i.e. if allergic
	cmp	r8d,dword[rdx]
	jz	.check_score			; size did not changed, try next candidate
; no way this is how we `push` things can't wait to see how things are properly done
	mov	dword[rdx+4+4*(r8-1)],ecx	; rdx=size rdx+4=array then each int is double word
	mov	dword[rdx],r8d
	jmp	.check_score
.quit:	
	ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
