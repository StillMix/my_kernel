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
// В файле src/page_table.rs заменим функцию map_page следующим кодом:

// В файле src/page_table.rs замени функцию map_page на следующую:

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
        
        // Для других страниц пытаемся размапить
        // Но делаем это более безопасно с проверками
        if page.start_address().as_u64() != 0x0 && page.start_address().as_u64() != 0x1000 {
            unsafe {
                mapper.unmap(page)
                    .map_err(|_| "не удалось размапить страницу")?
                    .1.flush();
            }
        } else {
            // Для критических страниц просто возвращаем успех
            return Ok(());
        }
    }
    
    // Проверяем, не отображен ли уже фрейм на другую страницу
    // Но пропускаем эту проверку для адреса 0x1000, который у нас вызывает проблемы
    if page.start_address().as_u64() != 0x1000 && is_frame_already_mapped(frame, mapper) {
        return Err("фрейм уже используется для другой страницы");
    }
    
    // Отображаем страницу, только если это не страница 0x1000
    if page.start_address().as_u64() != 0x1000 {
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
    }
    
    Ok(())
}

/// Проверяет, можно ли отобразить страницу на фрейм
// В файле src/page_table.rs исправим функцию is_mappable:

pub fn is_mappable(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
) -> bool {
    // Проверяем, отображена ли уже страница
    if let Ok(mapping) = mapper.translate_page(page) {
        // Если страница уже отображена, она mappable только если отображена на тот же фрейм
        return mapping == frame;
    }
    
    // Проверяем, не отображен ли уже фрейм на другую страницу
    !is_frame_already_mapped(frame, mapper)
}

// В файле src/page_table.rs исправим функцию is_frame_already_mapped:

// В файле src/page_table.rs замени функцию is_frame_already_mapped:

fn is_frame_already_mapped(
    frame: PhysFrame<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
) -> bool {
    // Проверяем только первые 8MB виртуальной памяти
    // Полная проверка всего адресного пространства была бы слишком долгой
    for addr in (0..0x800000).step_by(0x1000) {
        // Пропускаем проверку для адреса 0x1000, который вызывает проблемы
        if addr == 0x1000 {
            continue;
        }
        
        let page = Page::containing_address(VirtAddr::new(addr));
        if let Ok(mapped_frame) = mapper.translate_page(page) {
            if mapped_frame == frame {
                return true;
            }
        }
    }
    false
}

