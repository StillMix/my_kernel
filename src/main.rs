#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Модуль для работы с VGA-буфером
mod vga_buffer;
// Подключаем другие модули

// Точка входа в ядро
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Выводим сообщение "Привет, мир!"
    vga_buffer::print_string("Привет, мир!");
    
    // Зацикливаемся, чтобы ядро не завершилось
    loop {}
}

// Эта функция вызывается при панике
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}