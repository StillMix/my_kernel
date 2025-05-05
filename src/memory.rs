use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{
        FrameAllocator, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

/// Структура для определения физического адреса активной таблицы страниц уровня 4
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Создает новый аллокатор кадров из карты памяти загрузчика.
    ///
    /// Эта функция небезопасна, потому что вызывающая сторона должна гарантировать, что
    /// переданная карта памяти действительна. Вся память, возвращаемая аллокатором,
    /// должна быть неиспользуемой.
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Возвращает итератор по используемым фреймам из карты памяти.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // Получаем доступные регионы памяти из карты
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        // Отображаем каждый регион на его диапазон адресов
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // Преобразуем в итератор по 4K фреймам
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // Создаем `PhysFrame` объекты
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/// Возвращает мутабельную ссылку на активную таблицу страниц уровня 4
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}