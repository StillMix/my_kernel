#![no_std] // Не используем стандартную библиотеку
#![no_main] // Не используем стандартную точку входа

use my_os::println;

/// Эта функция вызывается при запуске ядра
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Выводим приветствие
    println!("Hello, World!");
    println!("Welcome to my OS!");
    println!("Press a key to test...");
    
    loop {
        // Пытаемся прочитать данные с клавиатуры
        use my_os::keyboard;
        use x86_64::instructions::port::Port;
        
        // Проверяем, есть ли данные от клавиатуры
        let mut status_port = Port::new(0x64);
        let status: u8 = unsafe { status_port.read() };
        
        // Если бит 0 установлен, значит есть данные для чтения
        if status & 1 == 1 {
            keyboard::handle_keyboard();
        }
    }
}

// /// Эта функция вызывается при панике
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     println!("{}", info);
//     loop {}
// }