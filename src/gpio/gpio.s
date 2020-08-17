.include "src/gpio/gpio.inc"
.equ GPIO_CTRL_ADDR,    0x10012000
.equ GPIO_OUTPUT_EN,    0x008
.equ GPIO_OUTPUT_VAL,   0x00C
.equ GPIO_OUTPUT_XOR,   0x040

.equ GPIO_RGB_PINS,     0x680000

.globl _gpio_init
_gpio_init:
    addi sp, sp, -16            
    sw ra, 12(sp)               

    li t0, GPIO_CTRL_ADDR       # Load the base GPIO address
    li t1, GPIO_RGB_PINS        # Get the RGP Pins offset
    sw t1, GPIO_OUTPUT_EN(t0)   # Enable RGB pins as output pins
    sw t1, GPIO_OUTPUT_XOR(t0)  # Set the XOR to that the pins are Active High
    sw x0, GPIO_OUTPUT_VAL(t0)  # Set all writable GPIO pins to zero

    li t1, GPIO_BLUE_LED        
    sw t1, GPIO_OUTPUT_VAL(t0)  # Write the new output so we turn on the requested LED

    lw ra, 12(sp)               # Restore the return address
    addi sp, sp, 16             # Deallocate stack frame
.globl _gpio_set
_gpio_set:
    addi sp, sp, -16            
    sw ra, 12(sp)

    li t0, GPIO_CTRL_ADDR       # Load the base GPIO address
    sw a0, GPIO_OUTPUT_VAL(t0)  # Set the requested PIN


