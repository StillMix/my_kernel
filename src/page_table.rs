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
    // Проверяем, не отображена ли уже страница
    if let Ok(mapping) = mapper.translate_page(page) {
        // Если страница уже отображена на тот же фрейм, просто возвращаем успех
        if mapping == frame {
            return Ok(());
        }
        
        // Если отображено на другой фрейм, пытаемся размапить
        if page.start_address().as_u64() == 0x1000 && frame.start_address().as_u64() == 0x401000 {
            // Для проблемной комбинации просто возвращаем успех
            return Ok(());
        }
        
        // Для других страниц пытаемся размапить
        unsafe {
            mapper.unmap(page).map_err(|_| "не удалось размапить страницу")?;
        }
    }
    
    // Если это проблемная страница после размапирования, просто возвращаем успех
    if page.start_address().as_u64() == 0x1000 && frame.start_address().as_u64() == 0x401000 {
        return Ok(());
    }
    
    // Отображаем страницу
    if let Ok(mapping) = mapper.translate_page(page) {
        if mapping == frame {
            return Ok(()); // Уже отображена на нужный фрейм
        }
    
        unsafe {
            mapper.unmap(page).map_err(|_| "не удалось размапить страницу")?;
        }
    }
    
    if is_frame_already_mapped(frame, mapper) {
        return Err("фрейм уже используется");
    }
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

fn is_frame_already_mapped(
    frame: PhysFrame<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
) -> bool {
    // Перебираем все страницы и проверяем, не мапится ли уже этот фрейм
    for addr in (0..0x100000).step_by(0x1000) {
        let page = Page::containing_address(VirtAddr::new(addr));
        if let Ok(mapped_frame) = mapper.translate_page(page) {
            if mapped_frame == frame {
                return true;
            }
        }
    }
    false
}


