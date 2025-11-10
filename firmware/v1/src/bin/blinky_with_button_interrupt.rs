#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    handler,
    timer::timg::TimerGroup,
};

extern crate alloc;

#[embassy_executor::task]
async fn run() {
    loop {
        defmt::println!("Hello World from Task");
        Timer::after(Duration::from_millis(1_000)).await
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    esp_println::logger::init_logger_from_env();
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer_group_zero = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timer_group_zero.timer0);

    let mut led = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());
    led.set_low();

    let button = Input::new(
        peripherals.GPIO22,
        InputConfig::default().with_pull(Pull::Up),
    );

    spawner.spawn(run()).ok();

    loop {
        if button.is_low() {
            led.toggle();
            defmt::println!("Togling Led");
        } else {
            led.set_high();
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[handler]
fn handler() {}
