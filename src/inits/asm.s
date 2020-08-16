.globl _start
_start:
    li sp,0x80004
    jal vk_start
    j .

.globl _dummy
_dummy:
    ret

.equ GPIO_RGB_PINS,     0x680000
.equ GPIO_CTRL_ADDR,    0x10012000
.equ GPIO_OUTPUT_EN,    0x008
.equ GPIO_OUTPUT_VAL,   0x00C
.equ GPIO_OUTPUT_XOR,   0x040

.equ GPIO_RGB_PINS,     0x680000
.equ GPIO_RED_LED,      0x400000
.equ GPIO_BLUE_LED,     0x200000
.equ GPIO_GREEN_LED,    0x080000

.globl _put_32
_put_32:
    sw a1,(a0)
    ret

.globl _get_32
_get_32:
    lw a0,(a0)
    ret

.globl _play
_play:
    addi sp, sp, -16            # Allocate Stack Frame
    sw ra, 12(sp)               # Save return address onto the stack

    li t0, GPIO_CTRL_ADDR       # Load the base GPIO address
    li t1, GPIO_RGB_PINS        # Get the RGP Pins offset
    sw t1, GPIO_OUTPUT_EN(t0)   # Enable RGB pins as output pins
    sw t1, GPIO_OUTPUT_XOR(t0)  # Set the XOR to that the pins are Active High
    sw x0, GPIO_OUTPUT_VAL(t0)  # Set all writable GPIO pins to zero

    li t1, GPIO_BLUE_LED        # Get the current value of the pins
    sw t1, GPIO_OUTPUT_VAL(t0)  # Write the new output so we turn on the requested LED

    lw ra, 12(sp)               # Restore the return address
    addi sp, sp, 16             # Deallocate stack frame
