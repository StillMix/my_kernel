#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo, config::Config};

// Определяем точку входа с правильной сигнатурой для нового загрузчика
entry_point!(kernel_main, config = CONFIG);

// Конфигурация загрузчика
static CONFIG: Config = {
    let mut config = Config::new();
    // Включаем минимальные настройки
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Получаем адрес VGA буфера
    let vga_buffer_ptr = 0xb8000 as *mut u8;
    
    // Сообщение для вывода
    let message = b"Hello, World!";
    
    // Выводим сообщение на экран
    unsafe {
        for (i, &byte) in message.iter().enumerate() {
            *vga_buffer_ptr.add(i * 2) = byte;
            *vga_buffer_ptr.add(i * 2 + 1) = 0x0F; // Белый цвет на черном фоне
        }
    }
    
    // Зацикливаемся, чтобы ядро не завершилось
    loop {}
}

/// Эта функция вызывается при панике
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}