#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)] 

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod keyboard;
pub mod interrupts;  
pub mod terminal;
pub mod gdt;  // Добавляем новый модуль

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("ПАНИКА: {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    serial_println!("Инициализация GDT...");
    gdt::init();
    
    serial_println!("Инициализация IDT...");
    interrupts::init_idt();
    
    serial_println!("Инициализация PIC...");
    unsafe {
        interrupts::PICS.lock().initialize();
        
        // Важно: разрешаем только прерывание клавиатуры
        // 0xFC вместо 0xFD для проверки - это разрешит IRQ0 (таймер) и IRQ1 (клавиатура)
        interrupts::PICS.lock().write_masks(0xFC, 0xFF);
    }
    
    serial_println!("Включение прерываний...");
    x86_64::instructions::interrupts::enable();
    
    serial_println!("Инициализация завершена");
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}
