/* File: linker.ld */
ENTRY(_start)

SECTIONS {
    /* Load the kernel at 1MB */
    . = 1M;

    .boot :
    {
        /* This ensures the header is physically first in the binary */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }
}
