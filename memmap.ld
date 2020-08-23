MEMORY
{
    rom : ORIGIN = 0x20010000, LENGTH = 0x1000
    ram : ORIGIN = 0x80000000, LENGTH = 0x4000
}
ENTRY(_start)
HIGH_MEM = ORIGIN(ram) + LENGTH(ram);
LOW_MEM = ORIGIN(ram);
RESET_VEC = 0x10040000;
SECTIONS
{
    .text : ALIGN(4)
    {
        . = ALIGN(4);
        *(.text*);
        . = ALIGN(4);
    } > rom
    .vt . : ALIGN(4)
    {
        . = ALIGN(4);
        .vt.begin = . ;
        *(.vt.*)
        .vt.reset = RESET_VEC;
        .vt.end = . ;
        . = ALIGN(4);
    } > rom
    .sys . : ALIGN(4)
    {
        . = ALIGN(4);
        .sys.begin = .;
        *(.sys.*)
        .sys.low_mem = LOW_MEM;
        .sys.high_mem = HIGH_MEM;
        .sys.end = .;
        . = ALIGN(4);
    } > rom
    .rodata : ALIGN(4) { *(.rodata*) } > rom
    .bss : ALIGN(4) { *(.bss*) } > ram
    .eh_frame (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        *(.eh_frame .eh_frame.*);
        . = ALIGN(4);
    } > rom
}