.text
.globl _start
_start:
    lui sp, %pcrel_hi(.sys.high_mem)
    jal ra, _set_mtvec
    jal ra, _gpio_init
    jal ra, _gpio_green
    # generate interupt.
    la a1, 0x20010FFD
    addi a0, x0, 1
    sw a0, (a1)
    j .
