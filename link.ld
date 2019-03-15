/** The first position in the generated file, 0x80000 **/
ENTRY(_start);

/** Sections in the memory */
SECTIONS
{
    /** Memory address to load the software */
    . = 0x80000;

    /** Start location */
    __start = .;

    /** Beginning of the read-only part CONST and code */
    __rodata_start = .;

    /** code */ 
    .text :
    {
        /** KEEP is to prevent eliminating dead code */
        KEEP(
            /** The boot code should be the first code to load (0x80000) */
            *(.text.boot)
        )
        /** Any other code */
        *(.text .text.*)
    }

    /** Exception handling vectors, Vector Base Address should be aligned to 2048 */
    .exception_vectors ALIGN(2048):
    {
        *(.exception_vectors)
    }

    /** Constants and static read-only declarations */
    .rodata :
    {
        *(.rodata .rodata.*)
    }

    /** Align to page size (4KiB), IMPORTANT for memory management */
    . = ALIGN(4096);

    /** End of read-only data and code */
    __rodata_end = .;

    /** Beginning of data section */
    __data_start = .;

    /** Compile-time initialised global variables */
    .data :
    {
        *(.data .data.*)
    }

    /** Align to page size (4KiB), IMPORTANT for memory management */
    . = ALIGN(4096);

    /** End of the data section */
    __data_end = .;

    /** Beginning of the bss section */
    __bss_start = .;

    /** Uninitialised global variables aligned to 64 bit, bss will be zeroed out */
    .bss ALIGN(8):
    {
        *(.bss .bss.*)
        /** Variables set to undefined */
        *(COMMON)
    }

    /** Throw away sections */
    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
    
    /** Align to page size (4KiB), IMPORTANT for memory management */
    . = ALIGN(4096);

    /** End of the bss section */
    __bss_end = .;

    /** End location */
    __end = .;
}
