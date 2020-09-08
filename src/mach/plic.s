.equ PLIC_BASE,     0x0C000008
.equ PLIC_PRIORITY, 0x00
.equ PLIC_ENABLE,   0x0C002000

.globl _set_plic
_set_plic:
    la t0, PLIC_ENABLE
    lw t1, 0(t0)
    ori t1, t1, 0x4
    sw t1, 0(t0)
	la t0, PLIC_BASE
    lw t1, PLIC_PRIORITY(t0)
    ori t1, t1, 0x7
    sw t1, PLIC_PRIORITY(t0)
    ret
