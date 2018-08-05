section .multiboot_header

header_start:
    dd 0xe85250d6                   ; magic number
    dd 0                            ; architecture (i386)
    dd header_end - header_start    ; header size
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))    ; checksum

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:
