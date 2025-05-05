#![no_std]
#![no_main]

use my_os::{println, print, serial_println};
use x86_64::instructions::port::Port;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Отладочный вывод
    serial_println!("Запуск ядра...");
    
    // Инициализируем систему
    serial_println!("Инициализация системы...");
    my_os::init();
    
    // Проверка состояния контроллера клавиатуры
    serial_println!("Проверка контроллера клавиатуры...");
    unsafe {
        let mut command_port = Port::new(0x64);
        let status: u8 = command_port.read();
        serial_println!("Статус контроллера клавиатуры: 0x{:02X}", status);
    }
    
    // Выводим приветствие
    serial_println!("Инициализация успешна!");
    println!("Добро пожаловать в MyOS!");
    println!("Введите 'help' для получения списка команд");
    print!("> ");
    
    // Главный цикл ОС
    serial_println!("Вход в главный цикл");
    loop {
        // Чтобы ЦП не нагревался
        x86_64::instructions::hlt();
    }
}