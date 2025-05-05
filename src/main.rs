#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod vga_buffer;
mod interrupts;
mod gdt;
mod serial; // Добавьте эту строку

#[macro_use]
extern crate lazy_static;

/// Обработчик паники
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("PANIC: {}", info);
    hlt_loop();
}

/// Точка входа
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Kernel started!");
    serial_println!("Kernel started via serial!");
    
    // Инициализируем GDT перед прерываниями
    gdt::init();
    
    println!("GDT initialized!");
    println!("Initializing interrupts...");
    
    // Инициализируем прерывания
    unsafe { interrupts::init(); }
    
    println!("Interrupts initialized!");
    println!("Keyboard is now active!");
    println!("Press any key to test input...");
    
    // Явно включаем прерывания (на всякий случай)
    x86_64::instructions::interrupts::enable();
    
    // Бесконечный цикл ожидания
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        // Используем интрукцию процессора HLT для экономии энергии
        x86_64::instructions::hlt();
    }
}