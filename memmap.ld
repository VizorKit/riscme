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
    .text : 
    {
        *(.text*);
    } > rom
    .vt . :
    {
        .vt.begin = . ;
        *(.vt.*)
        .vt.reset = RESET_VEC;
        .vt.end = . ;
    } > rom
    .sys . :
    {
        .sys.begin = .;
        *(.sys.*)
        .sys.low_mem = LOW_MEM;
        .sys.high_mem = HIGH_MEM;
        .sys.end = .;
    } > rom
    .rodata : { *(.rodata*) } > rom
    .bss : { *(.bss*) } > ram
    .eh_frame (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        *(.eh_frame .eh_frame.*);
        . = ALIGN(4);
    } > rom
}