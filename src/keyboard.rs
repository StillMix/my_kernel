use x86_64::instructions::port::Port;
use crate::{print, println};
use crate::terminal::Terminal;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TERMINAL: Mutex<Terminal> = Mutex::new(Terminal::new());
}

pub fn init() {
    // Инициализируем обработку прерываний от клавиатуры
    println!("Инициализация клавиатуры...");
}

// Функция для чтения скан-кода с порта клавиатуры
pub fn read_scancode() -> u8 {
    unsafe {
        let mut port = Port::new(0x60);
        port.read()
    }
}

// Преобразование скан-кода в ASCII
pub fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    match scancode {
        0x1E => Some(b'a'),
        0x30 => Some(b'b'),
        0x2E => Some(b'c'),
        0x20 => Some(b'd'),
        0x12 => Some(b'e'),
        0x21 => Some(b'f'),
        0x22 => Some(b'g'),
        0x23 => Some(b'h'),
        0x17 => Some(b'i'),
        0x24 => Some(b'j'),
        0x25 => Some(b'k'),
        0x26 => Some(b'l'),
        0x32 => Some(b'm'),
        0x31 => Some(b'n'),
        0x18 => Some(b'o'),
        0x19 => Some(b'p'),
        0x10 => Some(b'q'),
        0x13 => Some(b'r'),
        0x1F => Some(b's'),
        0x14 => Some(b't'),
        0x16 => Some(b'u'),
        0x2F => Some(b'v'),
        0x11 => Some(b'w'),
        0x2D => Some(b'x'),
        0x15 => Some(b'y'),
        0x2C => Some(b'z'),
        0x39 => Some(b' '), // Пробел
        0x1C => Some(b'\n'), // Enter
        0x0E => Some(8),   // Backspace
        _ => None,
    }
}

// Простой обработчик клавиатуры
pub fn handle_keyboard() {
    let scancode = read_scancode();
    
    // Преобразуем скан-код в ASCII и передаем его в терминал
    if let Some(ascii_code) = scancode_to_ascii(scancode) {
        TERMINAL.lock().input(ascii_code);
    }
}