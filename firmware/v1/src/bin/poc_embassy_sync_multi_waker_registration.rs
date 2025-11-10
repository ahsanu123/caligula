#![no_std]
#![no_main]

// ================================================================================
// MultiWakerRegistration - Utility registering and waking multiple Waker’s.
// ================================================================================

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
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

    let mut led1 = Output::new(peripherals.GPIO32, Level::Low, OutputConfig::default());
    led1.set_high();

    let mut led2 = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());
    led2.set_low();

    spawner.spawn(run()).ok();

    loop {
        led1.toggle();
        led2.toggle();

        defmt::println!("Togling Led");
        Timer::after(Duration::from_millis(3_000)).await;
    }
}
