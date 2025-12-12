mod args;
mod prompt;
mod serial_io;
mod terminal;

use args::Args;
use prompt::{prompt_baudrate, prompt_device};
use serial_io::run_loop;
use terminal::RawModeGuard;
use tokio_serial::SerialStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse_or_exit();

    let mut device = args.device.unwrap_or_default();
    let mut baud = args.baudrate;

    if device.is_empty() {
        device = prompt_device()?;
        baud = prompt_baudrate()?;
    }

    let mut serial = SerialStream::open(&tokio_serial::new(&device, baud))
        .map_err(|e| format!("Failed to open {} @ {} baud: {}", device, baud, e))?;

    println!("🚀 Serial Device Opened: {} @ {} baud", device, baud);
    println!("⌨️ Type characters to send | ❌ Press Ctrl+Q to quit");

    let _raw_mode = RawModeGuard::new()?;
    let result = run_loop(&mut serial).await;

    result
}
