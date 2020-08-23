.align 4
.globl _set_mtvec
_set_mtvec:
    csrrci t0, mstatus, 3                   # disable interrupts
    la a0, _fail_handler                    # loads reset vector
    csrrw t2, mtvec, a0                     # get address of reset
    csrrci t1, mtvec, 1                     # set to direct
    csrrsi t0, mstatus, 3                   # enable interrupts
    ret
.align 8
_fail_handler:
    jal ra, _gpio_red
    mret
