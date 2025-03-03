/*
 * Memory regions of PIC32MZEF devices
 *
 * LENGTH values need to be adapted to specific device variant.
 * REGION_ALIAS functions are used to put the startup code either into the boot
 * flash memory or into the program flash memory keeping the boot flash free for
 * a boot loader.
 */

/* Symbols used for interrupt-vector table generation */
PROVIDE(_vector_spacing = 0x0001);
PROVIDE(_ebase_address = 0x9D000000);  /* first 4 KiB of program flash */

MEMORY
{
    boot_flash          (rx)    : ORIGIN = 0xBFC00000, LENGTH = 64k
    program_flash       (rx)    : ORIGIN = 0x9D000000, LENGTH = 2M
    sram                (w!x)   : ORIGIN = 0x80000000, LENGTH = 512k
    /* boot flash 1 sequence and configuration words */
    configsfrs                  : ORIGIN = 0xBFC4FF40, LENGTH = 192
}

REGION_ALIAS("exception_mem", program_flash)
REGION_ALIAS("program_mem", program_flash)
REGION_ALIAS("data_mem", sram)

REGION_ALIAS(reset_mem, boot_flash)

