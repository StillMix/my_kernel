#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use my_kernel::println;
// Добавьте в импорты в начале функции kernel_main:
use x86_64::structures::paging::Mapper; // Убраны лишние скобки

entry_point!(kernel_main);

// Изменим функцию kernel_main
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use my_kernel::memory;
    use my_kernel::page_table;
    use x86_64::{structures::paging::{Page, PageTableFlags, PhysFrame}, PhysAddr, VirtAddr};

    println!("Привет, мир! Это моё ядро на Rust!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // Инициализация таблицы страниц
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // Отображаем критические страницы нашего ядра
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    println!("Таблица страниц и аллокатор фреймов инициализированы успешно!");
    
    println!("Инициализация системы управления памятью...");

    // Вместо жестко заданного адреса страницы и фрейма, выберем безопасные значения
    let test_page = Page::containing_address(VirtAddr::new(0x1000000)); // 16MB
    let test_frame = PhysFrame::containing_address(PhysAddr::new(0x600000)); // 6MB

    println!("Тестирование отображения страницы {:?} на фрейм {:?}...", test_page, test_frame);

    // Отображаем тестовую страницу
    match page_table::map_page(
        test_page,
        test_frame,
        flags,
        &mut mapper,
        &mut frame_allocator,
    ) {
        Ok(_) => println!("Страница успешно отображена!"),
        Err(e) => println!("Ошибка отображения страницы: {}", e),
    }

    // Проверяем, правильно ли выполнено отображение
    if let Ok(mapped_frame) = mapper.translate_page(test_page) {
        if mapped_frame == test_frame {
            println!("Проверка отображения: успешно!");
        } else {
            println!("Проверка отображения: ошибка! Страница отображена на неправильный фрейм.");
        }
    } else {
        println!("Проверка отображения: ошибка! Страница не отображена.");
    }

    // Безопасная работа с памятью теперь возможна

    #[cfg(test)]
    test_main();

    println!("Ядро запущено успешно!");
    loop {}
}

/// Эта функция вызывается при панике
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}