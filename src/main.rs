#![no_std]
#![no_main]

use my_os::{println, print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Отладочный вывод
    serial_println!("Запуск ядра...");
    
    // Инициализируем систему
    serial_println!("Инициализация системы...");
    my_os::init();
    
    // Выводим приветствие
    serial_println!("Инициализация успешна!");
    println!("Добро пожаловать в MyOS!");
    println!("Введите 'help' для получения списка команд");
    print!("> ");
    
    // Главный цикл ОС
    serial_println!("Вход в главный цикл");
    loop {
        x86_64::instructions::hlt();
    }
}