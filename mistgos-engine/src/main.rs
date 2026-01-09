// File: mistgos-engine/src/main.rs

#![no_std]
#![no_main]

// Import the new VGA buffer module to enable println!
pub mod vga_buffer;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

// Define the entry point for the 64-bit kernel
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // New way: Clear screen and print using the VGA driver
    println!("MISTGOS Engine is online.");
    println!("Graph-based Modular Operating System");
    println!("------------------------------------");

    // Count memory regions from the bootloader's memory map
    let mut region_count = 0;
    for _region in boot_info.memory_map.iter() {
        region_count += 1;
    }

    // Now we can print the actual number to the screen
    println!("Hardware: Found {} memory regions.", region_count);

    loop {}
}

// Only ONE panic handler is allowed in the entire binary
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // We use our new println! to see the error
    println!("{}", info);
    loop {}
}
