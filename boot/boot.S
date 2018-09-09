section .text
bits 32

extern kernel_main

boot:
    call kernel_main
    hlt

