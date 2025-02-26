/*
    Linker script for the PIC32MZ2048EFM144
*/

PROVIDE(_ebase = 0xBFC00200);

_RESET_ADDR         = 0xBFC00000;
_GEN_EXCPT_ADDR     = _ebase + 0x180;

MEMORY
{
    phys_prog_mem    : ORIGIN = 0x1D00000000, LENGTH = 0x0007FFFF
    phys_boot_flash  : ORIGIN = 0x1FC0000000, LENGTH = 0x00073FFF
    sfrs             : ORIGIN = 0x1F80000000, LENGTH = 0x000FFFFF
}