#![no_std]
#![no_main]

use my_os::serial_println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Booting OS...");

    // Инициализируем IDT
    serial_println!("Starting IDT initialization...");
    my_os::interrupts::init_idt();

    // Инициализируем PIC
    serial_println!("Starting PIC initialization...");
    unsafe {
        my_os::pic::init();
    }

    serial_println!("Enabling interrupts...");
    // Разрешаем прерывания
    x86_64::instructions::interrupts::enable();
    serial_println!("Interrupts enabled");

    serial_println!("System initialized. Press any key...");

    // Добавим счетчик циклов для отладки
    let mut loop_count = 0;

    loop {
        // Показываем, что система всё еще работает
        if loop_count % 100_000_000 == 0 {
            serial_println!(
                "Waiting for keyboard input... (loop {})",
                loop_count / 100_000_000
            );
        }
        loop_count += 1;

        // Ожидаем прерываний
        x86_64::instructions::hlt();
    }
}
