.section .text.init.mtvec
.align 2
.global _early_mtvec
_early_mtvec:
    csrr t0, mcause
    csrr t1, mepc
    csrr t2, mtval
    j _early_mtvec

.globl _set_mtvec
_set_mtvec:
    auipc t0, 0 