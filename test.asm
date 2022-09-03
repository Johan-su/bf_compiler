bits 64
default rel

segment .data
    msg db "Hello world!", 0xd, 0xa, 0

segment .text
global main
extern putchar
extern getchar

main:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 32

    mov rcx, 90
    call    putchar