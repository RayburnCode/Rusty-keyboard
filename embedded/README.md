<!-- @format -->

# EMBEDDED RUST WITH RASPBERRY PI

./flash.sh

**had to install Picotool for the host operating system**

[Picotool](https://github.com/raspberrypi/pico-sdk-tools/releases)

```code
sudo picotool info -a
```

Use THIS

- since we arent using a RP debugger hardware. We are using Picotool instead and thus need to run the flash with picotool

```
cargo build --release
elf2uf2-rs ../target/thumbv6m-none-eabi/release/embedded embedded.uf2
sudo picotool load ../target/thumbv6m-none-eabi/release/embedded -t elf
```

```
cargo build --release
# From embedded folder
elf2uf2-rs ../target/thumbv6m-none-eabi/release/embedded embedded.uf2
sudo picotool load embedded.uf2
sudo picotool reboot
```

## Development Boards

1. DWEII RP2040 Core Board w/ usb C for Raspberry PI Pico

2. Raspberry PI RP2040

- Dual-core ARM Cortex M0+ Processor
- SPI/12C/UART

### Platforms Supported

[Rust platform support](https://doc.rust-lang.org/beta/rustc/platform-support.html)
[Embedded Rust](https://docs.rust-embedded.org/book/)

#### ARM Cortex M0

thumbv6m-none-eabi

```rust
    rustup target add thumbv6m-none-eabi
```

- The target CPU option is cortex-m0plus.

```rust
cargo size -- -Ax
```

```
probe-rs chip list
```

```
cargo flash --release --chip RP2040
```

cargo embed --chip RP2040

```test
cargo tree -e features | grep -n "critical-section" -A2
```
