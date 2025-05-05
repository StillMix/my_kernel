// src/memory.rs
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        Size4KiB,
    },
    PhysAddr, VirtAddr,
};

/// Инициализирует новый OffsetPageTable.
///
/// Этот функция небезопасна, потому что вызывающий код должен гарантировать, что
/// полная физическая память отображена на переданный `physical_memory_offset`.
/// Кроме того, этот функция должна вызываться только один раз для избежания
/// неопределенного поведения.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Возвращает ссылку на активную таблицу 4 уровня.
///
/// Этот функция небезопасна, потому что вызывающий код должен гарантировать, что
/// полная физическая память отображена на переданный `physical_memory_offset`.
/// Также, эта функция должна вызываться только один раз для избежания
/// неопределенного поведения.
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // небезопасное
}

/// Временная структура для выделения фреймов из загрузочной информации
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

pub fn map_physical_region(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    flags: PageTableFlags,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), &'static str> {
    use crate::page_table;
    
    // Проверяем, можно ли отобразить
    if !page_table::is_mappable(page, frame, mapper) {
        return Err("Страница уже отображена на другой фрейм");
    }
    
    // Отображаем страницу
    page_table::map_page(page, frame, flags, mapper, frame_allocator)
}

/// Отображает диапазон виртуальных страниц на диапазон физических фреймов
pub fn map_physical_region_range(
    start_page: Page<Size4KiB>,
    start_frame: PhysFrame<Size4KiB>,
    page_count: usize,
    flags: PageTableFlags,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), &'static str> {
    for i in 0..page_count {
        let page = start_page + i as u64;
        let frame = start_frame + i as u64;
        map_physical_region(page, frame, flags, mapper, frame_allocator)?;
    }
    
    Ok(())
}


impl BootInfoFrameAllocator {
    /// Создает новый BootInfoFrameAllocator из переданной карты памяти.
    ///
    /// Эта функция небезопасна, потому что вызывающий код должен гарантировать, что переданная
    /// карта памяти корректна. Принципиальные фреймы, которые уже используются, не должны быть
    /// помечены как свободные в карте памяти.
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Возвращает итератор по доступным фреймам
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // Получаем области памяти из карты
        let regions = self.memory_map.iter();
        // Фильтруем доступные регионы
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        // Получаем адресные диапазоны из регионов
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // Преобразуем их в итератор по фреймам
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // Создаем `PhysFrame` объекты
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
