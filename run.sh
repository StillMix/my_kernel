#!/bin/bash
clear
cargo clean
cargo clean
cargo clean
clear
cargo bootimage
clear
qemu-system-x86_64 -drive format=raw,file=target/x86_64-my_os/debug/bootimage-my_os.bin -serial mon:stdio -no-reboot -d int -D qemu.log -device isa-debug-exit,iobase=0xf4,iosize=0x04