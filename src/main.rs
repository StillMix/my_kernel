#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod vga_buffer;
mod interrupts;

/// Обработчик паники
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

/// Точка входа
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Kernel started!");
    println!("Press any key...");
    
    // Инициализируем прерывания
    unsafe { interrupts::init(); }
    
    // Бесконечный цикл ожидания
    loop {
        x86_64::instructions::hlt();
    }
}