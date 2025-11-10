use core::convert::Infallible;
use embedded_io::ErrorType;
use esp_hal::{Blocking, uart::UartTx};

pub struct Writer(pub UartTx<'static, Blocking>);

impl ErrorType for Writer {
    type Error = Infallible;
}

impl embedded_io::Write for Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let res = self.0.write(buf).unwrap();
        Ok(res)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.0.flush().unwrap();
        Ok(())
    }
}

pub struct AppState {
    pub led_brightness: [u8; 4],
    pub num_commands: usize,
}
