use x86_64::instructions::port::Port;
use crate::{print, println};

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

// Простой обработчик клавиатуры
pub fn handle_keyboard() {
    let scancode = read_scancode();
    
    // Очень простое отображение скан-кодов
    match scancode {
        0x01 => println!("ESC нажата"),
        0x1E => println!("A нажата"),
        0x30 => println!("B нажата"),
        0x2E => println!("C нажата"),
        0x20 => println!("D нажата"),
        0x12 => println!("E нажата"),
        0x21 => println!("F нажата"),
        0x22 => println!("G нажата"),
        0x23 => println!("H нажата"),
        0x17 => println!("I нажата"),
        0x24 => println!("J нажата"),
        0x25 => println!("K нажата"),
        0x26 => println!("L нажата"),
        0x32 => println!("M нажата"),
        0x31 => println!("N нажата"),
        0x18 => println!("O нажата"),
        0x19 => println!("P нажата"),
        0x10 => println!("Q нажата"),
        0x13 => println!("R нажата"),
        0x1F => println!("S нажата"),
        0x14 => println!("T нажата"),
        0x16 => println!("U нажата"),
        0x2F => println!("V нажата"),
        0x11 => println!("W нажата"),
        0x2D => println!("X нажата"),
        0x15 => println!("Y нажата"),
        0x2C => println!("Z нажата"),
        0x39 => println!("Space нажата"),
        0x1C => println!("Enter нажата"),
        _ => print!("."),
    }
}