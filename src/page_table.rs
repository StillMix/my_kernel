use x86_64::{
    structures::paging::{
        PageTableFlags, PhysFrame, Size4KiB, FrameAllocator,
        mapper::MapToError, page::Page, mapper::MapperAllSizes, OffsetPageTable
    },
    VirtAddr,
};

/// Инициализирует новый OffsetPageTable
///
/// Эта функция небезопасна, потому что вызывающая сторона должна гарантировать, что
/// физический адрес передается корректно. Кроме того, эта функция должна вызываться 
/// только один раз для инициализации системы отображения памяти.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    use crate::memory;
    
    let level_4_table = memory::active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Создает отображение для указанной страницы на указанный фрейм
#[allow(dead_code)]
pub fn map_page(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    flags: PageTableFlags,
    mapper: &mut impl MapperAllSizes,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    // Обернём вызов map_to в блок unsafe
    let result = unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
    }?;
    result.flush();
    Ok(())
}