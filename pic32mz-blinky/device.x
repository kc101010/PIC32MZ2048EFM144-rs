/*
    Memory layout for PIC32M2048EFM144 device

*/


INPUT("PIC32MZ2048EFM144.ld");

ENTRY(_RESET_ADDR);
EXTERN(_GEN_EXCPT_ADDR);

PROVIDE(_stack = ORIGIN(phys_prog_mem) + LENGTH(phys_prog_mem));