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

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

pub fn init() {
    interrupts::init_idt();  // Инициализируем таблицу прерываний
    unsafe {
        // Настраиваем PIC (Programmable Interrupt Controller)
        let mut pic1_data: x86_64::instructions::port::Port<u8> = x86_64::instructions::port::Port::new(0x21);
        let _pic2_data: x86_64::instructions::port::Port<u8> = x86_64::instructions::port::Port::new(0xA1);
        
        // Разрешаем прерывания от клавиатуры (IRQ 1)
        let value = pic1_data.read(); // Сначала читаем значение
        pic1_data.write(value & !(1 << 1)); // Затем записываем модифицированное значение
        
        // Разрешаем общие прерывания
        x86_64::instructions::interrupts::enable();
    }
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