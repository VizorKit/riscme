.include "src/gpio/gpio.inc"
.equ GPIO_CTRL_ADDR,    0x10012000
.equ GPIO_OUTPUT_EN,    0x008
.equ GPIO_OUTPUT_VAL,   0x00C
.equ GPIO_OUTPUT_XOR,   0x040

.equ GPIO_RGB_PINS,     0x680000

.globl _gpio_init
_gpio_init:
    li t0, GPIO_CTRL_ADDR       # Load the base GPIO address
    li t1, GPIO_RGB_PINS        # Get the RGP Pins offset
    sw t1, GPIO_OUTPUT_EN(t0)   # Enable RGB pins as output pins
    sw t1, GPIO_OUTPUT_XOR(t0)  # Set the XOR to that the pins are Active High
    sw x0, GPIO_OUTPUT_VAL(t0)  # Set all writable GPIO pins to zero

    li t1, GPIO_BLUE_LED        
    sw t1, GPIO_OUTPUT_VAL(t0)  # Write the new output so we turn on the requested LED
    ret
.globl _gpio_red
_gpio_red:
    li t0, GPIO_CTRL_ADDR       # Load the base GPIO address
    sw x0, GPIO_OUTPUT_VAL(t0)
    li a0, GPIO_RED_LED
    sw a0, GPIO_OUTPUT_VAL(t0)  # Write the new output so we turn on the requested LED
    ret
.globl _gpio_green
_gpio_green:
    li t0, GPIO_CTRL_ADDR       # Load the base GPIO address
    sw x0, GPIO_OUTPUT_VAL(t0)
    li a0, GPIO_GREEN_LED
    sw a0, GPIO_OUTPUT_VAL(t0)  # Write the new output so we turn on the requested LED
    ret
.globl _gpio_blue
_gpio_blue:
    li t0, GPIO_CTRL_ADDR       # Load the base GPIO address
    sw x0, GPIO_OUTPUT_VAL(t0)
    li a0, GPIO_BLUE_LED
    sw a0, GPIO_OUTPUT_VAL(t0)  # Write the new output so we turn on the requested LED
    ret
       