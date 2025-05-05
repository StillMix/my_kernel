use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::{println, keyboard, serial_println};
use pic8259::ChainedPics; // Добавляем эту библиотеку

// Константы для PIC
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

// Перечисление для интерфейсов
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard, // PIC_1_OFFSET + 1
}

// Инициализация PIC
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("ИСКЛЮЧЕНИЕ: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    serial_println!("ИСКЛЮЧЕНИЕ: ДВОЙНАЯ ОШИБКА (код: {}):\n{:#?}", error_code, stack_frame);
    loop {
        // Просто бесконечный цикл, чтобы система не перезагружалась
        x86_64::instructions::hlt();
    }
}
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    serial_println!("Прерывание клавиатуры получено");
    
    // Просто считываем скан-код и отправляем EOI, без вызова сложной обработки
    let scancode: u8 = unsafe { x86_64::instructions::port::Port::new(0x60).read() };
    serial_println!("Получен скан-код: {}", scancode);
    
    // EOI - End of Interrupt
    unsafe {
        let mut pic1_cmd = x86_64::instructions::port::Port::new(0x20);
        pic1_cmd.write(0x20u8); // EOI сигнал
    }
    
    serial_println!("Обработка клавиатуры завершена");
}