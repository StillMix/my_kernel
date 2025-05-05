use crate::{print, println, vga_buffer};


const MAX_COMMAND_LENGTH: usize = 50;

pub struct Terminal {
    buffer: [u8; MAX_COMMAND_LENGTH],
    position: usize,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            buffer: [0; MAX_COMMAND_LENGTH],
            position: 0,
        }
    }

    pub fn input(&mut self, key: u8) {
        if key == b'\n' {
            self.execute_command();
            self.position = 0;
            print!("\n> ");
        } else if key == 8 && self.position > 0 { // Backspace
            self.position -= 1;
            self.buffer[self.position] = 0;
            print!("\x08 \x08"); // Стирает символ на экране
        } else if self.position < MAX_COMMAND_LENGTH - 1 && key >= 32 && key <= 126 {
            self.buffer[self.position] = key;
            self.position += 1;
            print!("{}", key as char);
        }
    }

    fn execute_command(&self) {
        let cmd = core::str::from_utf8(&self.buffer[0..self.position]).unwrap_or("Invalid UTF-8");
        
        println!("\nВыполнение команды: '{}'", cmd);
        
        match cmd {
            "help" => {
                println!("Доступные команды:");
                println!("  help - показать эту справку");
                println!("  clear - очистить экран");
                println!("  color - изменить цвет текста");
                println!("  reboot - перезагрузить систему");
            }
            "clear" => {
                // Очистка экрана
                for _ in 0..25 {
                    println!();
                }
            }
           "color" => {
    // Изменение цвета текста
    use vga_buffer::{WRITER, Color};
    WRITER.lock().set_color(Color::Yellow, Color::Black);
    println!("Цвет текста изменен на желтый");
}
            "reboot" => {
                println!("Перезагрузка...");
                unsafe {
                    use x86_64::instructions::port::Port;
                    let mut port = Port::new(0x64);
                    port.write(0xFEu8);
                }
            }
            _ => {
                println!("Неизвестная команда: '{}'", cmd);
            }
        }
    }
}