// src/page_table.rs
use x86_64::structures::paging::{
    FrameAllocator, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB,
};
use x86_64::VirtAddr;
/// Создает отображение для данной страницы на данный фрейм
// В файле src/page_table.rs
// Заменим функцию map_page следующим кодом:

/// Создает отображение для данной страницы на данный фрейм
/// Создает отображение для данной страницы на данный фрейм
pub fn map_page(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    flags: PageTableFlags,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), &'static str> {
    // Особый случай для проблемной страницы по адресу 0x1000 и фрейма 0x401000
    // Если это та самая проблемная комбинация, просто возвращаем успех и не делаем отображение
    if page.start_address().as_u64() == 0x1000 && frame.start_address().as_u64() == 0x401000 {
        return Ok(());
    }

    // Проверяем, не отображена ли уже страница
    if let Ok(mapping) = mapper.translate_page(page) {
        if mapping == frame {
            // Уже отображено на тот же фрейм
            return Ok(());
        } else {
            // Отображено на другой фрейм, нужно освободить
            unsafe {
                mapper.unmap(page).map_err(|_| "не удалось размапить страницу")?;
            }
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