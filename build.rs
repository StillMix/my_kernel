use std::{env, fs, path::PathBuf};

fn main() {
    // Регистрируем директорию сборки для bootloader
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let kernel = PathBuf::from(env::var("CARGO_PKG_NAME").unwrap());

    // Создаем директорию для загрузочного образа
    let uefi_path = out_dir.join("uefi.img");
    let bios_path = out_dir.join("bios.img");

    // Строим UEFI образ
    bootloader_api::builder::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_path)
        .unwrap();

    // Строим BIOS образ
    bootloader_api::builder::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();

    // Копируем образы в директорию target
    let target_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    fs::create_dir_all(format!("{}/target/bootimages", target_dir)).unwrap();
    fs::copy(&uefi_path, format!("{}/target/bootimages/uefi.img", target_dir)).unwrap();
    fs::copy(&bios_path, format!("{}/target/bootimages/bios.img", target_dir)).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}