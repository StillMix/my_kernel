#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use my_kernel::println;
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use my_kernel::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Привет, мир! Это моё ядро на Rust!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // Инициализация таблицы страниц
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // Теперь можно безопасно работать с памятью

    #[cfg(test)]
    test_main();

    println!("Ядро запущено успешно!");
    loop {}
}

/// Эта функция вызывается при панике
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
