#![no_std]
#![no_main]

// ====================================================
// Signal - Signaling latest value to a single consumer.
//
// producer will make apple every second,
// it will saved on buffer. on every 10 apples,
// producer will signaling batch consumer to take
// the apples.
//
// ====================================================

use core::cell::RefCell;

use alloc::vec::Vec;
use defmt::Format;
use embassy_sync::signal::Signal;
use esp_hal::rng::Rng;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use esp_backtrace as _;
use esp_hal::timer::timg::TimerGroup;
use static_cell::StaticCell;

extern crate alloc;

#[derive(Format, Clone)]
pub struct Apple {
    pub weight: u32,
    pub diameter: u32,
}

impl Apple {
    pub fn new(weight: u32, diameter: u32) -> Self {
        Self { weight, diameter }
    }
}

enum ConsumeCommand {
    EatNow,
    NotFilled,
}

type SharedRNG = Mutex<CriticalSectionRawMutex, Rng>;
type SharedAppleType = RefCell<Vec<Apple>>;

static RNG_MUTEX: StaticCell<SharedRNG> = StaticCell::new();
static APPLES: StaticCell<SharedAppleType> = StaticCell::new();
static EAT_SIGNAL: Signal<CriticalSectionRawMutex, ConsumeCommand> = Signal::new();

#[embassy_executor::task]
async fn apple_producer(rng_mutex: &'static SharedRNG, apples: &'static SharedAppleType) {
    loop {
        let mut rng_lock = rng_mutex.lock().await;
        let apple = Apple::new(rng_lock.random(), rng_lock.random());
        let apple_len = apples.borrow().len();
        apples.borrow_mut().push(apple.clone());

        defmt::println!("producing apple {:?}, {}", apple, apple_len);
        if apple_len >= 10 {
            EAT_SIGNAL.signal(ConsumeCommand::EatNow);
        }
        Timer::after(Duration::from_millis(1_000)).await
    }
}

#[embassy_executor::task]
async fn apple_consumer(apples: &'static SharedAppleType) {
    loop {
        let command = EAT_SIGNAL.wait().await;

        if let ConsumeCommand::EatNow = command {
            defmt::println!(
                "consuming batch apples, {:?}",
                apples.borrow().first_chunk::<1>()
            );
            defmt::println!("emptyng the apple!!");
            apples.borrow_mut().clear();
        }

        Timer::after(Duration::from_millis(500)).await
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    esp_println::logger::init_logger_from_env();
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer_group_zero = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timer_group_zero.timer0);

    let rng = Rng::new(peripherals.RNG);

    let rng_mutex = RNG_MUTEX.init(Mutex::new(rng));
    let apples_cell = APPLES.init(RefCell::new(Vec::new()));

    spawner.spawn(apple_producer(rng_mutex, apples_cell)).ok();
    spawner.spawn(apple_consumer(apples_cell)).ok();
}
