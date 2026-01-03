// File: mistgos-engine/src/main.rs

#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

// Defines the entry point function.
// The bootloader transfers control to this function after initializing 64-bit mode.
entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // The VGA text buffer is located at physical address 0xb8000.
    let vga_buffer = 0xb8000 as *mut u8;

    // Message to display.
    let msg = b"MISTGOS (64-bit) has loaded successfully!";

    for (i, &byte) in msg.iter().enumerate() {
        unsafe {
            // Write the ASCII byte.
            *vga_buffer.offset(i as isize * 2) = byte;
            // Write the color attribute (0xb = light cyan).
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    // Infinite loop to prevent the kernel from returning (which is undefined behavior).
    loop {}
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
