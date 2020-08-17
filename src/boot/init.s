.include "src/gpio/gpio.inc"

.text
.globl _start
_start:
    auipc sp, %pcrel_hi(.sys.high_mem)
    j _gpio_init
    la a0, GPIO_RED_LED
    j _gpio_set
    j .

