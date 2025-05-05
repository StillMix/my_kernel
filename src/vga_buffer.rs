// VGA-буфер расположен по фиксированному адресу в памяти
const VGA_BUFFER_ADDR: usize = 0xb8000;
const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HEIGHT: usize = 25;

// Цвета VGA
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// Символ с цветом для VGA-буфера
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ColoredChar {
    ascii_character: u8,
    color_code: u8,
}

// Функция для вывода строки на экран
pub fn print_string(s: &str) {
    let vga_buffer = unsafe { 
        core::slice::from_raw_parts_mut(
            VGA_BUFFER_ADDR as *mut ColoredChar, 
            VGA_BUFFER_WIDTH * VGA_BUFFER_HEIGHT
        ) 
    };
    
    // Очищаем экран
    for i in 0..(VGA_BUFFER_WIDTH * VGA_BUFFER_HEIGHT) {
        vga_buffer[i] = ColoredChar {
            ascii_character: b' ',
            color_code: create_color_code(Color::Black, Color::Black),
        };
    }
    
    // Выводим строку символ за символом
    for (i, byte) in s.bytes().enumerate() {
        // Останавливаемся, если дошли до конца экрана
        if i >= VGA_BUFFER_WIDTH * VGA_BUFFER_HEIGHT {
            break;
        }
        
        // Обработка только печатных ASCII символов
        let character = if byte >= 32 && byte < 128 { byte } else { b'?' };
        
        // Записываем символ в буфер
        vga_buffer[i] = ColoredChar {
            ascii_character: character,
            color_code: create_color_code(Color::White, Color::Black),
        };
    }
}

// Создаёт код цвета из цветов переднего и заднего плана
fn create_color_code(foreground: Color, background: Color) -> u8 {
    (background as u8) << 4 | (foreground as u8)
}