/*
    Linker script for the PIC32MZ2048EFM144
*/

PROVIDE(_ebase = 0xBFC00200);

_RESET_ADDR         = 0xBFC00000;
_GEN_EXCPT_ADDR     = _ebase + 0x180;

MEMORY
{
    //phys_prog_mem    : ORIGIN = 0x1D000000, LENGTH = 0x0007FFFF
    //phys_boot_flash  : ORIGIN = 0x1FC00000, LENGTH = 0x00073FFF
    //sfrs             : ORIGIN = 0x1F800000, LENGTH = 0x000FFFFF

    kseg1_boot_flash      : ORIGIN = 0xBFC00000, LENGTH = 0x00073FFF
    kseg1_prog_flash (rx) : ORIGIN = 0xBD000000, LENGTH = 0x0007FFFF
    kseg1_ram        (w!x): ORIGIN = 0xA0000000, LENGTH = 0x0001FFFF
    kseg1_sfrs            : ORIGIN = 0xBF800000, LENGTH = 0x000FFFFF
    cfg_sfrs              : ORIGIN = 0xBF800000, LENGTH = 0x600
}