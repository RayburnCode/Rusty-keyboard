#!/bin/bash
cargo build --release
sudo picotool load ../target/thumbv6m-none-eabi/release/embedded -t elf
sudo picotool reboot