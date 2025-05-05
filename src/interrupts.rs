use crate::serial_println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);

        // Очень простой обработчик клавиатуры
        idt[0x21].set_handler_fn(keyboard_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    serial_println!("IDT initialized");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

// Максимально упрощенный обработчик клавиатуры
// Обработчик прерывания от клавиатуры с расширенной отладкой
extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    // Сообщение о начале обработки прерывания
    serial_println!("Keyboard interrupt started");

    // Считывание скан-кода
    serial_println!("Reading scancode...");
    let scancode: u8;
    unsafe {
        let mut port = x86_64::instructions::port::Port::new(0x60);
        scancode = port.read();
    }

    // Вывод считанного скан-кода
    serial_println!("Scancode read: {:#x}", scancode);

    // Сообщение перед отправкой EOI
    serial_println!("Sending EOI...");

    // Подтверждаем обработку прерывания
    unsafe {
        let mut port = x86_64::instructions::port::Port::new(0x20);
        port.write(0x20_u8); // EOI (End of Interrupt)
    }

    // Сообщение о завершении обработки прерывания
    serial_println!("Keyboard interrupt completed");
}
