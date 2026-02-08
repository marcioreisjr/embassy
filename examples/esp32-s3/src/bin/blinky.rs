#![no_std]
#![no_main]

// ESP-IDF app descriptor required by the 2nd stage bootloader (v5.2.3+) and espflash v4+.
// esp-hal 0.23.x does not include this automatically; newer versions (1.0+) do.
#[repr(C)]
struct EspAppDesc {
    magic_word: u32,
    secure_version: u32,
    reserv1: [u32; 2],
    version: [u8; 32],
    project_name: [u8; 32],
    time: [u8; 16],
    date: [u8; 16],
    idf_ver: [u8; 32],
    app_elf_sha256: [u8; 32],
    min_efuse_blk_rev_full: u32,
    max_efuse_blk_rev_full: u32,
    reserv2: [u32; 18],
}

#[used]
#[link_section = ".flash.appdesc"]
#[export_name = "esp_app_desc"]
static ESP_APP_DESC: EspAppDesc = EspAppDesc {
    magic_word: 0xABCD5432,
    secure_version: 0,
    reserv1: [0; 2],
    version: [0; 32],
    project_name: [0; 32],
    time: [0; 16],
    date: [0; 16],
    idf_ver: [0; 32],
    app_elf_sha256: [0; 32],
    min_efuse_blk_rev_full: 0,
    max_efuse_blk_rev_full: 0xFFFF,
    reserv2: [0; 18],
};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::gpio::{Level, Output};
use esp_hal::timer::timg::TimerGroup;
use log::*;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default());
    info!("Hello World!");

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    // Heltec LoRa ESP-S3: LED on GPIO35, active high
    let mut led = Output::new(peripherals.GPIO35, Level::High);

    loop {
        info!("high");
        led.set_high();
        Timer::after(Duration::from_millis(250)).await;

        info!("low");
        led.set_low();
        Timer::after(Duration::from_millis(250)).await;
    }
}
