    .section .text.entry
    .global _start
_start:
    # SBI passes mhartid in a0
    la sp, stack0
    addi a0, a0, 1
    li t0, 4096
    mul a0, a0, t0
    add sp, sp, a0
    call kmain
