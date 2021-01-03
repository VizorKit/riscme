MEMORY
{
    rom : ORIGIN = 0x20010000, LENGTH = 0x1000
    ram : ORIGIN = 0x80000000, LENGTH = 0x4000
}
ENTRY(_start)
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
        .vt.end = . ;
    } > rom
    .sys . :
    {
        .sys.begin = .;
        .sys.end = .;
    } > rom
    .rodata : ALIGN(4) { *(.rodata*) } > rom
    .bss : ALIGN(4) { *(.bss*) } > ram
    .eh_frame (NOLOAD) : ALIGN(4)
    {
        *(.eh_frame .eh_frame.*);
    } > rom
}
