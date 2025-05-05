#!/bin/bash
clear
cargo clean
cargo clean
cargo clean
clear
cargo bootimage
clear
qemu-system-x86_64 -drive format=raw,file=target/x86_64-my_os/debug/bootimage-my_os.bin -serial stdio