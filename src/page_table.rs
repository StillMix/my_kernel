// src/page_table.rs
use x86_64::structures::paging::{
    FrameAllocator, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB,
};

/// Создает отображение для данной страницы на данный фрейм
pub fn map_page(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    flags: PageTableFlags,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), &'static str> {
    // Проверяем, не отображена ли уже страница
    if let Ok(mapping) = mapper.translate_page(page) {
        if mapping == frame {
            // Уже отображено на тот же фрейм
            return Ok(());
        } else {
            // Отображено на другой фрейм, нужно освободить
            mapper.unmap(page).map_err(|_| "не удалось размапить страницу")?;
        }
    }
    
    // Отображаем страницу
    let result = unsafe {
        mapper
            .map_to(
                page, 
                frame,
                flags, 
                frame_allocator
            )
            .map_err(|_| "не удалось отобразить страницу")?
    };
    
    // Обновляем TLB (Translation Lookaside Buffer)
    result.flush();
    
    Ok(())
}

/// Проверяет, можно ли отобразить страницу на фрейм
pub fn is_mappable(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
) -> bool {
    // Проверяем, отображена ли уже страница
    if let Ok(mapping) = mapper.translate_page(page) {
        return mapping == frame; // Если отображена на тот же фрейм, то можно считать mappable
    }
    
    // Если не отображена, то считаем mappable
    true
}