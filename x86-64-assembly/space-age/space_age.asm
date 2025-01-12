default rel

section .text

    planets equ 8

global age
; float age(enum planet planet, int seconds);
; `cvtsi2ss` converts seconds to float
; `xmm0` is the returned float, equivalent of `rax` for integers
; `divss` — divide scalar single precision floating-point values
age:
    cmp         edi, planets
    mov         ecx, edi
    lea         rax, [seconds_per_year]
    cvtsi2ss    xmm0, esi
    divss       xmm0, [rax + 4 * rcx]   ; `dd` is 4 bytes
    ret

section .rodata

seconds_per_year:
    dd  7600543.81992   ; Mercury
    dd  19414149.052176 ; Venus
    dd  31557600.0      ; Earth
    dd  59354032.69008  ; Mars
    dd  374355659.124   ; Jupiter
    dd  929292362.8848  ; Saturn
    dd  2651370019.3296 ; Uranus
    dd  5200418560.032  ; Neptune
