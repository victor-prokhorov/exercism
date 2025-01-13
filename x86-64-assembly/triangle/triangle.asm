section .text

global is_equilateral
; we pass a struct with sides to rdi, struct have 3 long so 8 bytes each
; https://www.cs.uaf.edu/2017/fall/cs301/reference/x86_64.html
; `movsd` — move or merge scalar double precision floating-point value
; https://www.felixcloutier.com/x86/movsd
; `ucomisd` — unordered compare scalar double precision floating-point values and set eflags
; https://www.felixcloutier.com/x86/ucomisd
; make sure to use `jb`, `jbe` and not `jl`, `jle` when comparing floats
is_equilateral:
    call        read_sides
    call        is_triangle
    test        rax, rax
    je          false
    ucomisd     xmm0, xmm1
    jne         false
    ucomisd     xmm1, xmm2
    jne         false
    jmp         true

global is_isosceles
is_isosceles:
    call        read_sides
    call        is_triangle
    test        rax, rax
    je          false
    ucomisd     xmm0, xmm1
    je          true
    ucomisd     xmm1, xmm2
    je          true
    ucomisd     xmm0, xmm2
    je          true
    jmp         false

global is_scalene
is_scalene:
    call        read_sides
    call        is_triangle
    test        rax, rax
    je          false
    ucomisd     xmm0, xmm1
    je          false
    ucomisd     xmm1, xmm2
    je          false
    ucomisd     xmm0, xmm2
    je          false
    jmp         true

is_triangle:
    pxor        xmm4, xmm4              ; we cannot `ucomisd` with immediate, thus we need a zero
    ucomisd     xmm0, xmm4              ; `a > 0.`
    jbe         false
    ucomisd     xmm1, xmm4              ; `b > 0.`
    jbe         false
    ucomisd     xmm2, xmm4              ; `c > 0.`
    jbe         false
    movsd       xmm4, xmm0              ; `a + b >= c`
    addsd       xmm4, xmm1
    ucomisd     xmm4, xmm2
    jb          false
    movsd       xmm4, xmm1              ; `b + c >= a`
    addsd       xmm4, xmm2
    ucomisd     xmm4, xmm0
    jb          false
    movsd       xmm4, xmm0              ; `a + c >= b`
    addsd       xmm4, xmm2
    ucomisd     xmm4, xmm1
    jb          false
    jmp         true

read_sides:
    movsd       xmm0, [rsp + 16]        ; two procedures deep `2 * 8`
    movsd       xmm1, [rsp + 24]        ; long `q` is 8 bytes
    movsd       xmm2, [rsp + 32]
    ret

false:
    xor         rax, rax
    ret

true:
    mov         rax, 1
    ret


%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
