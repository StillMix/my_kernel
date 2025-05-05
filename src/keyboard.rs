use x86_64::instructions::port::Port;
use crate::{print, println};

pub fn init() {
    // Инициализируем обработку прерываний от клавиатуры
    println!("Keyboard initialization...");
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
        0x01 => println!("ESC pressed"),
        0x1E => println!("A pressed"),
        0x30 => println!("B pressed"),
        0x2E => println!("C pressed"),
        0x20 => println!("D pressed"),
        0x12 => println!("E pressed"),
        0x21 => println!("F pressed"),
        0x22 => println!("G pressed"),
        0x23 => println!("H pressed"),
        0x17 => println!("I pressed"),
        0x24 => println!("J pressed"),
        0x25 => println!("K pressed"),
        0x26 => println!("L pressed"),
        0x32 => println!("M pressed"),
        0x31 => println!("N pressed"),
        0x18 => println!("O pressed"),
        0x19 => println!("P pressed"),
        0x10 => println!("Q pressed"),
        0x13 => println!("R pressed"),
        0x1F => println!("S pressed"),
        0x14 => println!("T pressed"),
        0x16 => println!("U pressed"),
        0x2F => println!("V pressed"),
        0x11 => println!("W pressed"),
        0x2D => println!("X pressed"),
        0x2C => println!("Z pressed"),
        0x15 => println!("Y pressed"),
        0x39 => println!("Space pressed"),
        0x1C => println!("Enter pressed"),
        _ => print!("."),
    }
}