use embedded_cli::Command;

#[derive(Debug, Command)]
pub enum LedCommand {
    /// Get current LED value
    Get,

    /// Set LED value
    Set {
        /// LED brightness
        value: u8,
    },
}

#[derive(Debug, Command)]
pub enum AdcCommand<'a> {
    /// Read ADC value
    Read {
        /// Print extra info
        #[arg(short = 'V', long)]
        verbose: bool,

        /// Sample count (16 by default)
        #[arg(long)]
        samples: Option<u8>,

        #[arg(long)]
        sampler: &'a str,
    },
}

#[derive(Debug, Command)]
pub enum BaseCommand<'a> {
    /// Control LEDs
    Led {
        /// LED id
        #[arg(long)]
        id: u8,

        #[command(subcommand)]
        command: LedCommand,
    },

    /// Control ADC
    Adc {
        /// ADC id
        #[arg(long)]
        id: u8,

        #[command(subcommand)]
        command: AdcCommand<'a>,
    },

    /// Show some status
    Status,
}
