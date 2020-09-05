.equ AONBASE,       0x10000000
.equ wdogcfg,       0x000
.equ wdogcount,     0x008
.equ wdogs,         0x010
.equ wdogfeed,      0x018
.equ wdogkey,       0x01C
.equ wdogcmp0,      0x020
.equ rtccfg,        0x040
.equ rtccountlo,    0x048
.equ rtccounthi,    0x04C
.equ rtcs,          0x050
.equ rtccmp0,       0x060
.equ lfrosccfg,     0x070
.equ lfclkmux,      0x07C
.equ AONCFG,        0x300

.globl _set_rccmp
_set_rccmp:
    li t0, AONBASE          # loads the AON base address
    lw t1, rtccfg(t0)       # loads the config address which is rtcscale
    li t2, 0x100F
    or t2, t1, t2         	# ori the rtcscale to seconds in t1, and enable
    sw t2, rtccfg(t0)       # store the new rtcscale

    lw t1, rtcs(t0)         # loads the current clock time
    addi t1, t1, 0x03       # adds 3 seconds to the current clock time
    sw t1, rtccmp0(t0)      # sets the time to compare 3 seconds higher
    ret
