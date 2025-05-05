use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr; // Добавляем импорт VirtAddr
use lazy_static::lazy_static;

// Константа для индекса в Таблице дескрипторов прерываний (IDT)
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Определение таблицы состояний задач
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // Выделяем отдельный стек для обработки исключения двойной ошибки
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = unsafe { &STACK as *const _ as u64 };
            // Преобразуем u64 в VirtAddr
            VirtAddr::new(stack_start + STACK_SIZE as u64)
        };
        tss
    };
}

// Структура для хранения селекторов сегментов
pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}

// Определение глобальной таблицы дескрипторов
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        
        // Добавляем дескриптор кода
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        
        // Добавляем дескриптор таблицы состояний задач
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        
        // Возвращаем таблицу и селекторы
        (gdt, Selectors { code_selector, tss_selector })
    };
}

// Функция для инициализации GDT
pub fn init() {
    use x86_64::instructions::segmentation::{CS, Segment};
    use x86_64::instructions::tables::load_tss;
    
    // Загружаем GDT в процессор
    GDT.0.load();
    
    // Обновляем селекторы сегментов
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}