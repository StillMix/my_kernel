use crate::print;

use lazy_static::lazy_static;
use crate::serial_println;
use spin::Mutex;
use x86_64::instructions::port::Port;
// Buffer size for storing keyboard input
const BUFFER_SIZE: usize = 64;

lazy_static! {
    // Input buffer to store characters
    static ref INPUT_BUFFER: Mutex<KeyboardBuffer> = Mutex::new(KeyboardBuffer::new());
}

// Keyboard buffer structure
pub struct KeyboardBuffer {
    buffer: [u8; BUFFER_SIZE],
    position: usize,
}

impl KeyboardBuffer {
    pub fn new() -> Self {
        KeyboardBuffer {
            buffer: [0; BUFFER_SIZE],
            position: 0,
        }
    }

    // Print the prompt character
    pub fn print_prompt(&self) {
        print!("> ");
    }

    // Add a character to the buffer
    pub fn add_char(&mut self, c: u8) {
        if self.position < BUFFER_SIZE - 1 {
            self.buffer[self.position] = c;
            self.position += 1;
            self.buffer[self.position] = 0; // Null-terminate
            print!("{}", c as char);
        }
    }

    // Handle backspace - remove last character
    pub fn backspace(&mut self) {
        if self.position > 0 {
            self.position -= 1;
            self.buffer[self.position] = 0;

            // Use VGA buffer's backspace method
            use crate::vga_buffer::WRITER;
            WRITER.lock().backspace();
        }
    }

    // Process the Enter key - show the completed word
    pub fn process_line(&mut self) {
        serial_println!("");
        if self.position > 0 {
            serial_println!(
                "You typed: {}",
                core::str::from_utf8(&self.buffer[0..self.position]).unwrap_or("Invalid UTF-8")
            );
            self.clear();
        }
        // Print prompt for the next line
        self.print_prompt();
    }

    // Clear the buffer
    pub fn clear(&mut self) {
        self.position = 0;
        self.buffer = [0; BUFFER_SIZE];
    }
}

pub fn init() {
    // Initialize keyboard interrupt handling
    serial_println!("Keyboard initialization...");
    // Print the initial prompt
    INPUT_BUFFER.lock().print_prompt();
}

// Function to read scancode from keyboard port
pub fn read_scancode() -> u8 {
    unsafe {
        let mut port = Port::new(0x60);
        port.read()
    }
}

// Map scancodes to ASCII characters
fn scancode_to_ascii(scancode: u8) -> Option<u8> {
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
        0x39 => Some(b' '), // Space
        0x02 => Some(b'1'),
        0x03 => Some(b'2'),
        0x04 => Some(b'3'),
        0x05 => Some(b'4'),
        0x06 => Some(b'5'),
        0x07 => Some(b'6'),
        0x08 => Some(b'7'),
        0x09 => Some(b'8'),
        0x0A => Some(b'9'),
        0x0B => Some(b'0'),
        _ => None,
    }
}

// Simple keyboard handler
// Simple keyboard handler
pub fn handle_keyboard() {
    let scancode = read_scancode();

    // Обрабатываем только отпускание клавиш (release) чтобы избежать дублирования
    if scancode & 0x80 != 0 {
        return;
    }

    // Отладочный вывод
    serial_println!("Scancode: {:#02x}", scancode);

    // Handle special keys and regular characters
    match scancode {
        0x01 => serial_println!("ESC pressed"),     // ESC
        0x0E => INPUT_BUFFER.lock().backspace(),    // Backspace
        0x1C => INPUT_BUFFER.lock().process_line(), // Enter
        _ => {
            // Handle regular characters
            if let Some(ascii) = scancode_to_ascii(scancode) {
                INPUT_BUFFER.lock().add_char(ascii);
            }
        }
    }
}
