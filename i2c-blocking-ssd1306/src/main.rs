#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(panic_info_message)]

use embassy_executor::Spawner;
use embassy_rp::{gpio, i2c, uart};
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use panic_halt as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let config = uart::Config::default();
    let mut uart = uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config);
    let mut led = Output::new(p.PIN_25, Level::Low);

    uart.blocking_write("Program start!\r\n".as_bytes())
        .unwrap();

    let sda = p.PIN_8;
    let scl = p.PIN_9;

    let interface = I2CDisplayInterface::new(i2c::I2c::new_blocking(
        p.I2C0,
        scl,
        sda,
        i2c::Config::default(),
    ));
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();
    display.init().unwrap();
    let _ = display.clear();

    for char in "Display works!".chars() {
        let _ = display.print_char(char);
    }

    loop {
        led.set_high();
        uart.blocking_write("LED on\r\n".as_bytes()).unwrap();
        Timer::after(Duration::from_secs(1)).await;
        led.set_low();
        uart.blocking_write("LED off\r\n".as_bytes()).unwrap();
        Timer::after(Duration::from_secs(1)).await;
    }
}
