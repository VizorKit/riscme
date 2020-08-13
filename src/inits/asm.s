.globl _start
_start:
    lui x2,0x80004
    jal vk_start
    j .

.globl _dummy
_dummy:
    ret

.globl _put_32
_put_32:
    sw x11,(x10)
    ret

.globl _get_32
_get_32:
    lw x10,(x10)
    ret
