.align 4
.globl _set_mtvec
_set_mtvec:
    csrrci t0, mstatus, 3                   # disable interrupts
    la a0, _fail_handler                    # loads fail handler
    csrrw t2, mtvec, a0                     # get address of fail handler
    csrrci t1, mtvec, 1                     # set to direct mode
    csrrsi t0, mstatus, 3                   # enable interrupts
    ret
.align 8
_fail_handler:
    addi t0, x0, 0xFFFFFFFF
    csrrs t1, mcause, x0
    la t2, 0xFFFFFFFD
    and t2, t2, t1
    bne t2, t0, _red
    jal ra, _gpio_blue
    mret
_red:    
    jal ra, _gpio_red
    mret
