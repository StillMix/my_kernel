use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
fn utf8_to_cp866(c: char) -> u8 {
    match c {
        // Русские буквы с альтернативными кодами
        'А' => 128, 'Б' => 129, 'В' => 130, 'Г' => 131, 'Д' => 132, 'Е' => 133, 'Ж' => 134, 'З' => 135,
        'И' => 136, 'Й' => 137, 'К' => 138, 'Л' => 139, 'М' => 140, 'Н' => 141, 'О' => 142, 'П' => 143,
        'Р' => 144, 'С' => 145, 'Т' => 146, 'У' => 147, 'Ф' => 148, 'Х' => 149, 'Ц' => 150, 'Ч' => 151,
        'Ш' => 152, 'Щ' => 153, 'Ъ' => 154, 'Ы' => 155, 'Ь' => 156, 'Э' => 157, 'Ю' => 158, 'Я' => 159,
        'а' => 160, 'б' => 161, 'в' => 162, 'г' => 163, 'д' => 164, 'е' => 165, 'ж' => 166, 'з' => 167,
        'и' => 168, 'й' => 169, 'к' => 170, 'л' => 171, 'м' => 172, 'н' => 173, 'о' => 174, 'п' => 175,
        'р' => 224, 'с' => 225, 'т' => 226, 'у' => 227, 'ф' => 228, 'х' => 229, 'ц' => 230, 'ч' => 231,
        'ш' => 232, 'щ' => 233, 'ъ' => 234, 'ы' => 235, 'ь' => 236, 'э' => 237, 'ю' => 238, 'я' => 239,
        'ё' => 241, 'Ё' => 240,
        // Для остальных символов используем ASCII
        c if c.is_ascii() => c as u8,
        // Неизвестные символы заменяем на ?
        _ => b'?',
    }
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                // Используем метод update вместо write
                self.buffer.chars[row][col].update(|c| {
                    *c = ScreenChar {
                        ascii_character: byte,
                        color_code: self.color_code,
                    }
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            match c {
                // ASCII characters and line feed
                '\n' => self.new_line(),
                c if c.is_ascii() && c as u8 >= 0x20 && c as u8 <= 0x7e => self.write_byte(c as u8),
                // For non-ASCII characters, just print a question mark
                _ => self.write_byte(b'?'),
            }
        }
    }
// Таблица соответствий символов UTF-8 кодам CP866 (только для русских букв)

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                // Получаем символ, считывая его значение
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            // Используем метод write_value вместо write
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}