<!-- @format -->

# EMBEDDED RUST WITH RASPBERRY PI

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
