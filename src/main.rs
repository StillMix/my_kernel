#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod vga_buffer;
mod interrupts;
mod gdt; // Добавить импорт модуля GDT

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
    
    // Инициализируем GDT перед прерываниями
    gdt::init();
    
    println!("GDT initialized!");
    println!("Initializing interrupts...");
    
    // Инициализируем прерывания
    unsafe { interrupts::init(); }
    
    println!("Interrupts initialized!");
    println!("Press any key to test keyboard input...");
    
    // Бесконечный цикл ожидания
    loop {
        x86_64::instructions::hlt();
    }
}


// Добавьте эту функцию в конец файла
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}