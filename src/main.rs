#![no_std] // don't link the Rust standard library
#![feature(abi_x86_interrupt)] // x86_interrupt規約で定義された関数を使用する
#![no_main] // disable all Rust-level entry points

use x86_64::registers::control::Cr3;

mod gdt;
mod interrupts;
mod vga_buffer;

fn init() {
    gdt::init();
    interrupts::init_idt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    debug!("{:?}", _start as *const u8);
    unsafe {
        asm!("int3", options(nomem, nostack));
    }

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    fn stack_overflow() {
        stack_overflow(); // 再帰呼び出しのために、リターンアドレスがプッシュされる
    }
    stack_overflow();

    // // ページフォルトを起こす
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    debug!("Hello, world!");

    // x86_64::instructions::interrupts::int3();

    panic!("Some panic message");
    // assert_eq!(1, 2);

    loop {}
}

use core::{arch::asm, panic::PanicInfo};
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}
