# RESET_VECTOR should be 0x1004
.globl _reset
_reset:
    auipc t0, 0
    lw t1, -4(t0)           # loads the pin state into t1
    slli t1, t1, 0x3        # multiplies the pin state by 8
    add t0, t0, t1          # adds multiplied pin state to 0x1004
    lw t0, 252(t0)          # loads address value of t0 offset by 252 into t0
    jr t0                   # jumps to t0 with ra.
