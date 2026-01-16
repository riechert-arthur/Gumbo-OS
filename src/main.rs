#![no_std]
#![no_main]
/// no_std: Don't include the standard library
///
/// The kernel will not have access to the std library binary
/// because it has operating system constructs for networking,
/// memory allocation, and file systems, which will need a custom
/// implementation for Gumbo OS.
///
/// no_main: Don't use main as the entry point
///
/// The Rust runtime relies on the crt0 library to setup the stack
/// for a C program. It uses start as the entry point. We will not have
/// access to this in our kernel.

use core::panic::PanicInfo;

static GREETING: &[u8] = b"Welcome to Gumbo OS!";

/// Reimplement the panic handler
///
/// Normally the Rust std library defines the panic handler for us.
/// However, since the kernel will not have the std library at start up,
/// we will need to define our own versions of std library features.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Define a new entrypoint for Gumbo OS
/// 
/// no_mangle: Prevents rust from generating a unique id for the function
/// extern "C": Ensures we use the C calling conventions
/// It also shouldn't return, and it will rely on an exit system call the OS will implement
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    let vga_buf = 0xb8000 as *mut u8;

    for(i, &byte) in GREETING.iter().enumerate() {
        unsafe {
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xb;
        } 
    }
    
    loop {}
}
