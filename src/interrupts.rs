use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::{println, keyboard};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[0x21].set_handler_fn(keyboard_interrupt_handler); // Прерывание клавиатуры
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("ИСКЛЮЧЕНИЕ: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("ИСКЛЮЧЕНИЕ: ДВОЙНАЯ ОШИБКА\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Обработка прерывания клавиатуры
    keyboard::handle_keyboard();
    
    // Необходимо отправить сигнал EOI (End of Interrupt) контроллеру прерываний
    unsafe {
        use x86_64::instructions::port::Port;
        let mut port = Port::new(0x20);
        port.write(0x20u8); // EOI
    }
}