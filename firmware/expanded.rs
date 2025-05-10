#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    handler, timer::timg::TimerGroup,
};
extern crate alloc;
#[doc(hidden)]
async fn __run_task() {
    loop {
        {
            match () {
                () => {
                    defmt::export::acquire_header_and_release(
                        &{
                            defmt::export::make_istr({
                                #[link_section = ".defmt.{\"package\":\"caligula\",\"tag\":\"defmt_println\",\"data\":\"Hello World from Task\",\"disambiguator\":\"17998942867037545179\",\"crate_name\":\"blinky_with_button_interrupt\"}"]
                                #[export_name = "{\"package\":\"caligula\",\"tag\":\"defmt_println\",\"data\":\"Hello World from Task\",\"disambiguator\":\"17998942867037545179\",\"crate_name\":\"blinky_with_button_interrupt\"}"]
                                static DEFMT_LOG_STATEMENT: u8 = 0;
                                &DEFMT_LOG_STATEMENT as *const u8 as u16
                            })
                        },
                    );
                }
            }
        };
        Timer::after(Duration::from_millis(1_000)).await
    }
}
fn run() -> ::embassy_executor::SpawnToken<impl Sized> {
    const POOL_SIZE: usize = 1;
    static POOL: ::embassy_executor::_export::TaskPoolRef = ::embassy_executor::_export::TaskPoolRef::new();
    unsafe { POOL.get::<_, POOL_SIZE>()._spawn_async_fn(move || __run_task()) }
}
#[doc(hidden)]
#[doc(hidden)]
async fn ____embassy_main_task(spawner: Spawner) {
    {
        let peripherals = esp_hal::init(esp_hal::Config::default());
        esp_println::logger::init_logger_from_env();
        {
            static mut HEAP: core::mem::MaybeUninit<[u8; 72 * 1024]> = core::mem::MaybeUninit::uninit();
            unsafe {
                ::esp_alloc::HEAP
                    .add_region(
                        ::esp_alloc::HeapRegion::new(
                            HEAP.as_mut_ptr() as *mut u8,
                            72 * 1024,
                            ::esp_alloc::MemoryCapability::Internal.into(),
                        ),
                    );
            }
        };
        let timer_group_zero = TimerGroup::new(peripherals.TIMG0);
        esp_hal_embassy::init(timer_group_zero.timer0);
        let mut led = Output::new(
            peripherals.GPIO4,
            Level::Low,
            OutputConfig::default(),
        );
        led.set_low();
        let button = Input::new(
            peripherals.GPIO22,
            InputConfig::default().with_pull(Pull::Up),
        );
        spawner.spawn(run()).ok();
        loop {
            if button.is_low() {
                led.toggle();
                {
                    match () {
                        () => {
                            defmt::export::acquire_header_and_release(
                                &{
                                    defmt::export::make_istr({
                                        #[link_section = ".defmt.{\"package\":\"caligula\",\"tag\":\"defmt_println\",\"data\":\"Togling Led\",\"disambiguator\":\"15193900276124521299\",\"crate_name\":\"blinky_with_button_interrupt\"}"]
                                        #[export_name = "{\"package\":\"caligula\",\"tag\":\"defmt_println\",\"data\":\"Togling Led\",\"disambiguator\":\"15193900276124521299\",\"crate_name\":\"blinky_with_button_interrupt\"}"]
                                        static DEFMT_LOG_STATEMENT: u8 = 0;
                                        &DEFMT_LOG_STATEMENT as *const u8 as u16
                                    })
                                },
                            );
                        }
                    }
                };
            } else {
                led.set_high();
            }
            Timer::after(Duration::from_millis(100)).await;
        }
    }
}
#[doc(hidden)]
fn __embassy_main(spawner: Spawner) -> ::embassy_executor::SpawnToken<impl Sized> {
    const POOL_SIZE: usize = 1;
    static POOL: ::embassy_executor::_export::TaskPoolRef = ::embassy_executor::_export::TaskPoolRef::new();
    unsafe {
        POOL.get::<_, POOL_SIZE>()
            ._spawn_async_fn(move || ____embassy_main_task(spawner))
    }
}
#[doc(hidden)]
unsafe fn __make_static<T>(t: &mut T) -> &'static mut T {
    ::core::mem::transmute(t)
}
#[doc(hidden)]
#[export_name = "main"]
pub unsafe extern "C" fn __xtensa_lx_rt_main_trampoline() {
    __xtensa_lx_rt_main()
}
#[allow(clippy::inline_always)]
#[inline(always)]
fn __xtensa_lx_rt_main() -> ! {
    let mut executor = ::esp_hal_embassy::Executor::new();
    let executor = unsafe { __make_static(&mut executor) };
    executor
        .run(|spawner| {
            spawner.must_spawn(__embassy_main(spawner));
        })
}
extern "C" fn __esp_hal_internal_handler() {}
#[allow(non_upper_case_globals)]
const handler: esp_hal::interrupt::InterruptHandler = esp_hal::interrupt::InterruptHandler::new(
    __esp_hal_internal_handler,
    esp_hal::interrupt::Priority::min(),
);
