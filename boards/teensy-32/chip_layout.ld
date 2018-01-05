/**
 * All these data are gathered from:
 *
 *  K20 Sub-Family Reference Manual (K20P64M72SF1RM.pdf)
 *  Section 3.5.1.2
/**
 * ROM_ORIGIN and ROM_LENGTH define the memory region given to the kernel.
 *
 * Teensy 3.2/3.5/3.6 boards run the HalfKay bootloader on an external Cortex-M0, so
 * there is no need to allocate space for a bootloader unless there is a reason
 * to specifically use tockloader.
 *
 *  Section 3.5.1.2
 */
ROM_ORIGIN  = 0x00000000;
ROM_LENGTH  = 0x0003FFFF; /* Ends at 0x0003FFFF */

/**
 * Program flash is M total.
 *
 * [Kinetis K20 Sub-Family Reference Manual Section 3.5.1.2]
 */
PROG_ORIGIN = 0x10000000;
PROG_LENGTH = 0x10007FFF; /*Ends at 0x10007FFF */

/**
 * SRAM is 64K total, split into an upper 32K chunk and a lower 32K chunk.
 *
 * [Kinetis K20 Sub-Family Reference Manual Section 3.5.1.2]
 */
RAM_ORIGIN  = 0x14000000;
RAM_LENGTH  = 0x140007FF; /* End at 0x140007FF */

/**
 * Minimum memory protection unit alignment size.
 * [Kinetis K20 Sub-Family Reference Manual, 12.2.17?]
 *
 * TODO: verify this number
 */
MPU_MIN_ALIGN = 8K;
