#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;

extern crate alloc;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        defmt::trace!("trace");
        defmt::debug!("debug");
        defmt::info!("info");
        defmt::warn!("warn");
        defmt::error!("error");
        defmt::println!("looop.....");

        Timer::after(Duration::from_secs(5)).await;
    }
}
