use crate::serial_println;
use x86_64::instructions::port::Port;

// Порты для PIC
const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

// Команды для PIC
const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;
pub fn io_wait() {
    unsafe {
        let mut port = Port::new(0x80);
        port.write(0_u8);
    }
}
// Инициализация PIC
// Инициализация PIC
pub unsafe fn init() {
    serial_println!("Starting PIC initialization...");
    
    // Инициализация
    let mut port1_command = Port::new(PIC1_COMMAND);
    let mut port2_command = Port::new(PIC2_COMMAND);
    let mut port1_data = Port::new(PIC1_DATA);
    let mut port2_data = Port::new(PIC2_DATA);
    
    // Сохраняем маски
    let mask1 = port1_data.read();
    let mask2 = port2_data.read();
    serial_println!("Original PIC masks: {:#x}, {:#x}", mask1, mask2);
    
    // Инициализируем PIC
    serial_println!("Sending initialization command...");
    port1_command.write(ICW1_INIT);
    io_wait();
    port2_command.write(ICW1_INIT);
    io_wait();
    
    // Установка смещений векторов прерываний
    serial_println!("Setting interrupt vector offsets...");
    port1_data.write(0x20); // Master PIC starts at 0x20
    io_wait();
    port2_data.write(0x28); // Slave PIC starts at 0x28
    io_wait();
    
    // Настройка каскадирования
    serial_println!("Setting up cascading...");
    port1_data.write(4); // Master has a slave on IRQ2
    io_wait();
    port2_data.write(2); // Slave's cascade identity
    io_wait();
    
    // 8086 mode
    serial_println!("Setting 8086 mode...");
    port1_data.write(ICW4_8086);
    io_wait();
    port2_data.write(ICW4_8086);
    io_wait();
    
    // Маскируем все прерывания, кроме клавиатуры
    serial_println!("Setting interrupt masks...");
    port1_data.write(0xFF & !(1 << 1)); // Unmask only keyboard IRQ1
    port2_data.write(0xFF);             // Mask all slave interrupts
    
    // Проверяем установленные маски
    let new_mask1 = port1_data.read();
    let new_mask2 = port2_data.read();
    serial_println!("New PIC masks: {:#x}, {:#x}", new_mask1, new_mask2);
    
    serial_println!("PIC initialization complete");
}