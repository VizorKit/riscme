.include "src/gpio/gpio.inc.s"
.align 4
.equ GPIO_CTRL_ADDR,    0x10012000
.equ GPIO_OUTPUT_EN,    0x008
.equ GPIO_OUTPUT_VAL,   0x00C
.equ GPIO_OUTPUT_XOR,   0x040
.equ GPIO_RGB_PINS,     0x680000

.macro set_gpio color
    li t0, GPIO_CTRL_ADDR       # Load the base GPIO address
    sw x0, GPIO_OUTPUT_VAL(t0)  # Set the current value to 0
    li a0, \color               # Load the requested color
    sw a0, GPIO_OUTPUT_VAL(t0)  # Set gpio to requested color
.endm

.globl _gpio_init
_gpio_init:
    li t0, GPIO_CTRL_ADDR       # Load the base GPIO address
    li t1, GPIO_RGB_PINS        # Get the RGP Pins offset
    sw t1, GPIO_OUTPUT_EN(t0)   # Enable RGB pins as output pins
    sw t1, GPIO_OUTPUT_XOR(t0)  # Set the XOR to that the pins are Active High
    sw x0, GPIO_OUTPUT_VAL(t0)  # Set all writable GPIO pins to zero

    li t1, GPIO_BLUE_LED        # Load blue to indicate ready
    sw t1, GPIO_OUTPUT_VAL(t0)  # Write the new output so we turn on the requested LED
    ret
.globl _gpio_red
_gpio_red:
    set_gpio GPIO_RED_LED
    ret
.globl _gpio_green
_gpio_green:
    set_gpio GPIO_GREEN_LED
    ret
.globl _gpio_blue
_gpio_blue:
    set_gpio GPIO_BLUE_LED
    ret
