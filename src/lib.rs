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
    serial_println!("ПАНИКА: {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
pub fn init() {
    serial_println!("Инициализация IDT...");
    interrupts::init_idt();
    
    serial_println!("Инициализация PIC...");
    
    // Инициализация PIC (Programmable Interrupt Controller)
    unsafe {
        // Данные порты для PIC1 и PIC2
        let mut pic1_cmd: x86_64::instructions::port::Port<u8> = x86_64::instructions::port::Port::new(0x20);
        let mut pic1_data: x86_64::instructions::port::Port<u8> = x86_64::instructions::port::Port::new(0x21);
        let mut pic2_cmd: x86_64::instructions::port::Port<u8> = x86_64::instructions::port::Port::new(0xA0);
        let mut pic2_data: x86_64::instructions::port::Port<u8> = x86_64::instructions::port::Port::new(0xA1);
        
        // Инициализация PIC
        // ICW1: начать инициализацию
        pic1_cmd.write(0x11);
        pic2_cmd.write(0x11);
        
        // ICW2: определение базовых векторов прерываний
        pic1_data.write(0x20); // IRQ 0-7: 0x20-0x27
        pic2_data.write(0x28); // IRQ 8-15: 0x28-0x2F
        
        // ICW3: соединение Master/Slave
        pic1_data.write(0x04); // PIC1 имеет подчиненный PIC на IRQ2 (бит 2)
        pic2_data.write(0x02); // PIC2 является подчиненным с индексом 2
        
        // ICW4: другие настройки
        pic1_data.write(0x01); // 8086/88 режим
        pic2_data.write(0x01); // 8086/88 режим
        
        // Временно отключаем все прерывания для отладки
        pic1_data.write(0xFF); // Маскируем все прерывания на PIC1
        pic2_data.write(0xFF); // Маскируем все прерывания на PIC2
    }
    
    serial_println!("Включение прерываний...");
    x86_64::instructions::interrupts::enable();
    
    // Инициализация клавиатуры отключена для отладки
    // keyboard::init();
    
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
