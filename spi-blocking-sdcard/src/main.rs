#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(panic_info_message)]

use embassy_executor::Spawner;
use embassy_rp::{gpio, spi, uart};
use embassy_time::{Delay, Duration, Timer};
use gpio::{Level, Output};
use panic_halt as _;

use embedded_sdmmc;
use heapless;
use ufmt;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let config = uart::Config::default();
    let mut uart = uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config);
    let mut led = Output::new(p.PIN_25, Level::Low);

    uart.blocking_write("Program start!\r\n".as_bytes())
        .unwrap();

    let spi_clk = p.PIN_18;
    let spi_tx = p.PIN_19;
    let spi_rx = p.PIN_20;
    let spi_cs = p.PIN_17;
    let spi_cs = Output::new(spi_cs, Level::Low);
    // create SPI
    let mut config = spi::Config::default();
    config.frequency = 400_000;
    let sdmmc_spi = spi::Spi::new_blocking(p.SPI0, spi_clk, spi_tx, spi_rx, config);

    let sdcard = embedded_sdmmc::SdCard::new(sdmmc_spi, spi_cs, Delay);
    let size = sdcard.num_bytes().unwrap();
    let mut sdcard_size_string: heapless::String<16> = heapless::String::new();
    uart.blocking_write("SD card reported size: ".as_bytes())
        .unwrap();
    ufmt::uwrite!(&mut sdcard_size_string, "{}", size).unwrap();
    uart.blocking_write(sdcard_size_string.as_bytes()).unwrap();
    uart.blocking_write(" bytes\r\n".as_bytes()).unwrap();

    let t = sdcard.get_card_type();
    let sd_success = match t {
        Some(c) => match c {
            embedded_sdmmc::sdcard::CardType::SD1 => "SD1\r\n",
            embedded_sdmmc::sdcard::CardType::SD2 => "SD2\r\n",
            embedded_sdmmc::sdcard::CardType::SDHC => "SDHC\r\n",
        },
        None => "Failed to get card type\r\n",
    };
    uart.blocking_write("SD card type: ".as_bytes()).unwrap();
    uart.blocking_write(sd_success.as_bytes()).unwrap();

    loop {
        led.set_high();
        uart.blocking_write("LED on\r\n".as_bytes()).unwrap();
        Timer::after(Duration::from_secs(1)).await;
        led.set_low();
        uart.blocking_write("LED off\r\n".as_bytes()).unwrap();
        Timer::after(Duration::from_secs(1)).await;
    }
}
