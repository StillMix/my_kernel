#!/bin/bash
cargo clean
cargo clean
cargo clean
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-my_os/debug/bootimage-my_os.bin -serial stdio