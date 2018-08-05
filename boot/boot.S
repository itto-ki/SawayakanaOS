section .text
bits 32

extern kernel_main

boot:
    mov dword [0xb8000], 0x2f6f2f4c
    mov dword [0xb8004], 0x2f642f61
    mov dword [0xb8008], 0x2f642f65
    mov dword [0xb800c], 0x2f212f21
    call kernel_main
    hlt

