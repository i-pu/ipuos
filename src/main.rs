#![no_std] // don't link the Rust standard library
#![feature(abi_x86_interrupt)]
#![no_main] // disable all Rust-level entry points

mod interrupts;
mod vga_buffer;

use interrupts::*;
use vga_buffer::*;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_idt();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    let ptr = 0xdeadbeaf as *mut u32;
    unsafe {
        *ptr = 42;
    }

    debug!("Hello, world!");
    panic!("Some panic message");

    #[cfg(test)]
    test_main();

    loop {}
}

use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}
