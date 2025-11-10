#![no_std]
#![no_main]

use cli::cli_led::command::{AdcCommand, BaseCommand, LedCommand};
use cli::cli_led::command_callback::{on_adc, on_led, on_status};
use cli::cli_led::writer::Writer;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, signal::Signal};
use embassy_time::{Delay, Timer};
use embedded_cli::Command;
use embedded_cli::cli::CliBuilder;
use embedded_cli::cli::CliHandle;
use esp_backtrace as _;
use esp_hal::{
    Blocking,
    timer::timg::TimerGroup,
    uart::{AtCmdConfig, Config, RxConfig, Uart, UartRx, UartTx},
};
use static_cell::StaticCell;

// fifo_full_threshold (RX)
const READ_BUF_SIZE: usize = 64;
// EOT (CTRL-D)
const AT_CMD: u8 = 0x04;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_println::println!("Init!");

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    let (tx_pin, rx_pin) = (peripherals.GPIO1, peripherals.GPIO3);

    let config = Config::default()
        .with_rx(RxConfig::default().with_fifo_full_threshold(READ_BUF_SIZE as u16));

    let mut uart0 = Uart::new(peripherals.UART0, config)
        .unwrap()
        .with_tx(tx_pin)
        .with_rx(rx_pin);

    uart0.set_at_cmd(AtCmdConfig::default().with_cmd_char(AT_CMD));

    let (mut rx, mut tx) = uart0.split();

    let writer = Writer(tx);

    let (command_buffer, history_buffer) = unsafe {
        static mut COMMAND_BUFFER: [u8; 40] = [0; 40];
        static mut HISTORY_BUFFER: [u8; 41] = [0; 41];
        #[allow(static_mut_refs)]
        (COMMAND_BUFFER.as_mut(), HISTORY_BUFFER.as_mut())
    };

    let mut cli = embedded_cli::cli::CliBuilder::default()
        .writer(writer)
        .command_buffer(command_buffer)
        .history_buffer(history_buffer)
        .build()
        .unwrap();

    // Create global state, that will be used for entire application
    let mut state = cli::cli_led::writer::AppState {
        led_brightness: [0; 4],
        num_commands: 0,
    };
    let _ = cli.write(|writer| {
        // storing big text in progmem
        // for small text it's usually better to use normal &str literals
        ufmt::uwrite!(
            writer,
            "{}",
            "Cli is running.
     Type \"help\" for a list of commands.
     Use backspace and tab to remove chars and autocomplete.
     Use up and down for history navigation.
     Use left and right to move inside input."
        )?;
        Ok(())
    });

    let rx_buff = &mut [0u8; 100];
    loop {
        // arduino_hal::delay_ms(10);
        // led.toggle();

        let _ = rx.read(rx_buff);
        // let _ = tx.write(rx_buff);
        //
        // Timer::after_secs(1).await;
        // Process incoming byte
        // Command type is specified for autocompletion and help
        // Processor accepts closure where we can process parsed command
        // we can use different command and processor with each call

        for &mut b in rx_buff {
            let _ = cli.process_byte::<BaseCommand<'_>, _>(
                b,
                &mut BaseCommand::processor(|cli, command| match command {
                    BaseCommand::Led { id, command } => on_led(cli, &mut state, id, command),
                    BaseCommand::Adc { id, command } => on_adc(cli, &mut state, id, command),
                    BaseCommand::Status => on_status(cli, &mut state),
                }),
            );
        }
    }

    // static SIGNAL: StaticCell<Signal<NoopRawMutex, usize>> = StaticCell::new();
    // let signal = &*SIGNAL.init(Signal::new());

    // spawner.spawn(reader(rx, &signal)).ok();
    // spawner.spawn(writer(tx, &signal)).ok();
}

// #[embassy_executor::task]
// async fn writer(mut tx: UartTx<'static, Blocking>, signal: &'static Signal<NoopRawMutex, usize>) {
//     use core::fmt::Write;
//     embedded_io::Write::write(
//         &mut tx,
//         b"Hello async serial. Enter something ended with EOT (CTRL-D).\r\n",
//     )
//     .unwrap();
//     embedded_io::Write::flush(&mut tx).unwrap();
//     loop {
//         let bytes_read = signal.wait().await;
//         signal.reset();
//         write!(&mut tx, "\r\n-- received {} bytes --\r\n", bytes_read).unwrap();
//         embedded_io::Write::flush(&mut tx).unwrap();
//     }
// }
//
// #[embassy_executor::task]
// async fn reader(mut rx: UartRx<'static, Blocking>, signal: &'static Signal<NoopRawMutex, usize>) {
//     const MAX_BUFFER_SIZE: usize = 10 * READ_BUF_SIZE + 16;
//
//     let mut rbuf: [u8; MAX_BUFFER_SIZE] = [0u8; MAX_BUFFER_SIZE];
//     let mut offset = 0;
//     loop {
//         let r = embedded_io::Read::read(&mut rx, &mut rbuf[offset..]);
//         match r {
//             Ok(len) => {
//                 offset += len;
//                 esp_println::println!("Read: {len}, data: {:?}", &rbuf[..offset]);
//                 offset = 0;
//                 signal.signal(len);
//             }
//             Err(e) => esp_println::println!("RX Error: {:?}", e),
//         }
//     }
// }
