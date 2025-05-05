use crate::gdt;
use crate::print;
use crate::println; // Добавляем импорт макроса println!
use crate::serial_println;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
// Константы для PIC
const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;

// Офсеты для PIC
const PIC1_OFFSET: u8 = 32;
const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

// Порты для PIC
struct Pic {
    offset: u8,
    command: Port<u8>,
    data: Port<u8>,
}

impl Pic {
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.offset <= interrupt_id && interrupt_id < self.offset + 8
    }

    unsafe fn end_of_interrupt(&mut self) {
        self.command.write(0x20);
    }
}

struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command: Port::new(PIC1_COMMAND),
                    data: Port::new(PIC1_DATA),
                },
                Pic {
                    offset: offset2,
                    command: Port::new(PIC2_COMMAND),
                    data: Port::new(PIC2_DATA),
                },
            ],
        }
    }

    pub unsafe fn initialize(&mut self) {
        // Сохраняем маски
        let mask1 = self.pics[0].data.read();
        let mask2 = self.pics[1].data.read();

        // Начало инициализации в каскадном режиме
        self.pics[0].command.write(ICW1_INIT);
        self.pics[1].command.write(ICW1_INIT);

        // Установка офсетов
        self.pics[0].data.write(self.pics[0].offset);
        self.pics[1].data.write(self.pics[1].offset);

        // Указываем PIC1, что существует ведомый PIC на линии 2
        self.pics[0].data.write(4);
        // Указываем PIC2, что он является ведомым на линии 2
        self.pics[1].data.write(2);

        // Установка режима 8086
        self.pics[0].data.write(ICW4_8086);
        self.pics[1].data.write(ICW4_8086);

        // Восстанавливаем маски
        self.pics[0].data.write(mask1);
        self.pics[1].data.write(mask2);
    }

    pub unsafe fn write_masks(&mut self, mask1: u8, mask2: u8) {
        self.pics[0].data.write(mask1);
        self.pics[1].data.write(mask2);
    }

    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.pics[1].handles_interrupt(interrupt_id) {
            self.pics[1].end_of_interrupt();
        }
        self.pics[0].end_of_interrupt();
    }
}

// Индексы прерываний
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC1_OFFSET,
    Keyboard, // PIC1_OFFSET + 1
}

// Контроллер прерываний
static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

// Таблица дескрипторов прерываний
// Добавьте в lazy_static! блок IDT следующие обработчики
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[InterruptIndex::Timer as usize].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_interrupt_handler);

        // Добавляем обработчик двойного сбоя
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // Добавляем обработчики базовых исключений
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
}

// И добавьте сами обработчики
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame);
    hlt_loop();
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2; // Перемещаем импорт сюда

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

// Добавьте функцию hlt_loop
fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// Инициализация прерываний
pub unsafe fn init() {
    println!("Loading IDT...");
    IDT.load();

    println!("Initializing PIC...");
    // Инициализируем PIC
    PICS.lock().initialize();

    println!("Setting PIC masks...");
    // Маскируем все прерывания, кроме клавиатуры и таймера
    // 0xFC = 1111 1100 - разрешаем только первые два прерывания (0 и 1)
    PICS.lock().write_masks(0xFC, 0xFF);

    println!("Enabling hardware interrupts...");
    // Разрешаем прерывания
    x86_64::instructions::interrupts::enable();

    println!("Interrupt initialization completed successfully!");
}

// Обработчик прерывания таймера
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    println!("EXCEPTION: DOUBLE FAULT");
    println!("{:#?}", stack_frame);
    hlt_loop();
}
// Обработчик прерывания клавиатуры

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::DecodedKey;
    use x86_64::instructions::port::Port;

    // Вывод отладочной информации
    serial_println!("Keyboard interrupt received!");

    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut keyboard = KEYBOARD.lock();
        let mut port = Port::new(0x60);

        let scancode: u8 = unsafe { port.read() };

        // Вывод скан-кода
        serial_println!("Scancode: {}", scancode);

        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        serial_println!("Pressed key: {}", character);
                        print!("{}", character);
                    }
                    DecodedKey::RawKey(key) => {
                        serial_println!("Pressed raw key: {:?}", key);
                        print!("{:?}", key);
                    }
                }
            }
        }

        unsafe {
            PICS.lock()
                .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
        }
    });
}
