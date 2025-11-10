use crate::cli_led::command::{AdcCommand, LedCommand};
use crate::cli_led::writer::{AppState, Writer};

use core::convert::Infallible;
use embedded_cli::cli::CliHandle;
use ufmt::uwrite;
use ufmt::uwriteln;

pub fn on_led(
    cli: &mut CliHandle<'_, Writer, Infallible>,
    state: &mut AppState,
    id: u8,
    command: LedCommand,
) -> Result<(), Infallible> {
    state.num_commands += 1;

    if id as usize > state.led_brightness.len() {
        uwrite!(cli.writer(), "{}{}{}", "LED", id, " not found")?;
    } else {
        match command {
            LedCommand::Get => {
                uwrite!(
                    cli.writer(),
                    "{}{}{}{}",
                    "Current LED",
                    id,
                    " brightness: ",
                    state.led_brightness[id as usize]
                )?;
            }
            LedCommand::Set { value } => {
                state.led_brightness[id as usize] = value;
                uwrite!(
                    cli.writer(),
                    "{}{}{}{}",
                    "Setting LED",
                    id,
                    " brightness to ",
                    state.led_brightness[id as usize]
                )?;
            }
        }
    }

    Ok(())
}

pub fn on_adc(
    cli: &mut CliHandle<'_, Writer, Infallible>,
    state: &mut AppState,
    id: u8,
    command: AdcCommand<'_>,
) -> Result<(), Infallible> {
    state.num_commands += 1;

    match command {
        AdcCommand::Read {
            verbose,
            samples,
            sampler,
        } => {
            let samples = samples.unwrap_or(16);
            if verbose {
                cli.writer().write_str("Performing sampling with ")?;
                cli.writer().write_str(sampler)?;
                uwriteln!(cli.writer(), "{}{}{}", "\nUsing ", samples, " samples")?;
            }
            uwrite!(
                cli.writer(),
                "{}{}{}{}",
                "Current ADC",
                id,
                " readings: ",
                43
            )?;
        }
    }
    Ok(())
}

pub fn on_status(
    cli: &mut CliHandle<'_, Writer, Infallible>,
    state: &mut AppState,
) -> Result<(), Infallible> {
    state.num_commands += 1;
    uwriteln!(cli.writer(), "{}{}", "Received: ", state.num_commands)?;
    Ok(())
}
