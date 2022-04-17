#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(alloc)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::panic::PanicInfo;

// use alloc::vec::Vec;

pub mod allocator;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_vga(HELLO);
    let a = b"aaa";
    print_vga(a);

    // let v = Vec::from([1, 2, 3, 4, 5]);

    let a: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    loop {}
}

fn print_vga(s: &[u8]) {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in s.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}
