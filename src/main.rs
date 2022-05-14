#![no_std] // don't link the Rust standard library
#![feature(abi_x86_interrupt)] // x86_interrupt規約で定義された関数を使用する
#![no_main] // disable all Rust-level entry points

use bootloader::{entry_point, BootInfo};
use x86_64::{
    structures::paging::{Page, Translate},
    VirtAddr,
};

mod gdt;
mod interrupts;
mod memory;
mod vga_buffer;

fn init() {
    gdt::init();
    interrupts::init_idt();
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    debug!("offset: {:?}", &phys_mem_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let addresses = [
        // 恒等対応しているVGAバッファのページ
        0xb8000,
        // コードページのどこか
        0x201008,
        // スタックページのどこか
        0x0100_0020_1a10,
        kernel_main as u64,
        // 物理アドレス "0" にマップされている仮想アドレス
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    // debug!("{:?}", kernel_main as *const u8);
    // unsafe {
    //     asm!("int3", options(nomem, nostack));
    // }

    // let (level_4_page_table, _) = Cr3::read();
    // println!(
    //     "Level 4 page table at: {:?}",
    //     level_4_page_table.start_address()
    // );

    // // stackを無限に使っても大丈夫か
    // // fn stack_overflow() {
    // //     stack_overflow(); // 再帰呼び出しのために、リターンアドレスがプッシュされる
    // // }
    // // stack_overflow();

    // // ページフォルトを起こす
    // unsafe {
    //     // caused by write
    //     // *(0x207f07 as *mut u64) = 42;
    //     // caused by read
    //     let a = *(0x207f07 as *mut u64);
    //     debug!("{}", a);
    // };

    // debug!("Hello, world!");

    // x86_64::instructions::interrupts::int3();

    // assert_eq!(1, 2);

    loop {}
}

use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
