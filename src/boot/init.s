.align 4
.text
.globl _start
_start:
    lui sp, %pcrel_hi(.sys.high_mem)
    jal ra, _set_mtvec
    jal ra, _gpio_init
    jal ra, _gpio_green
    jal ra, _set_rccmp
    jal ra, _set_plic
    # wait for interupt.
    wfi
    j .
