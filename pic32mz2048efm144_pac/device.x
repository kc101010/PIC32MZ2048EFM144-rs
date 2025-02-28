/*
    Memory layout for PIC32M2048EFM144 device

*/


INPUT("memory.x");

ENTRY(_RESET_ADDR);
EXTERN(_GEN_EXCPT_ADDR);

PROVIDE(_stack = ORIGIN(kseg1_prog_flash) + LENGTH(kseg1_prog_flash));

SECTIONS
{
    .cfg_sfrs : {
        KEEP(*(.kseg1_cfg_sfrs));
    } > cfg_sfrs

    .reset : 
    {
        KEEP(*(.reset));
        KEEP(*(.reset_startup));
    } > kseg1_boot_flash

    .text :
    {
        *(.text);
        *(.text.*);
    } > kseg1_prog_flash

    .rodata : ALIGN(4)
    {
        *(.rodata);
        *(.rodata.*);
        . = ALIGN(4);
    }  > kseg1_prog_flash

    .data : ALIGN(4)
    {
        *(.data);
        *(.data.*);
        . = ALIGN(4);
    } > kseg1_ram AT > kseg1_prog_flash

    __sdata = ADDR(.data);
    __edata = ADDR(.data) + SIZEOF(.data);

    .bss : ALIGN(4)
    {
        *(.bss)
        *(.bss.*)
        . = ALIGN(4);
    } > kseg1_ram

    __sbss = ADDR(.bss);
    __ebss = ADDR(.bss) + SIZEOF(.bss);

    __sheap = ADDR(.bss) + SIZEOF(.bss);

    .stack_sizes (INFO) :
    {
        KEEP(*(.stack_sizes));
    }

    /DISCARD/ :
    {
        *(.reginfo);
        *(.MIPS.abiflags);
    }

}