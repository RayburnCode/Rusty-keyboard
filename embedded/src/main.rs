#![no_std]
#![no_main]

use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::{
    pac,
    Clock,
    clocks::{init_clocks_and_plls},
    gpio::Pins,
    Sio,
    watchdog::Watchdog
};
use embedded_hal::digital::OutputPin;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    
    let _clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();
    
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);
    
    // Explicitly use GPIO 25
    let mut led_pin = pins.gpio25.into_push_pull_output();
    
    loop {
        // Turn on for 2 seconds
        led_pin.set_high().unwrap();
        for _ in 0..16_000_000 {
            cortex_m::asm::nop();
        }
        
        // Turn off for 2 seconds  
        led_pin.set_low().unwrap();
        for _ in 0..16_000_000 {
            cortex_m::asm::nop();
        }
    }
}