use clap::Parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialStream;

#[derive(Parser, Debug)]
#[command(version, about = "Serial port communication utility", long_about = None)]
struct Args {
    #[cfg_attr(target_os = "linux", clap(short, long))]
    #[cfg_attr(not(target_os = "linux"), clap(short, long))]
    port: Option<String>,

    #[clap(short, long, default_value_t = 115200)]
    baudrate: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = match Args::try_parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let mut port_name = match args.port {
        Some(p) => p,
        None => "".to_string(),
    };
    let mut baud_rate = args.baudrate;

    if port_name.is_empty() {
        port_name = prompt_port()?;
        baud_rate = prompt_baudrate()?;
    }

    let mut serial =
        SerialStream::open(&tokio_serial::new(&port_name, baud_rate)).map_err(|e| {
            format!(
                "Failed to open serial port '{}' at {} baud: {}",
                port_name, baud_rate, e
            )
        })?;
    println!("Opened {} at {} baud", port_name, baud_rate);
    println!("Type characters to send. ctrl+q to exit.");

    enable_raw_mode()?;

    let result = run_loop(&mut serial).await;

    disable_raw_mode()?;

    result
}

fn prompt_port() -> Result<String, Box<dyn std::error::Error>> {
    use std::io::Write;

    #[cfg(not(target_os = "windows"))]
    print!("Enter serial port (default: /dev/ttyUSB0): ");
    #[cfg(target_os = "windows")]
    print!("Enter serial port (default: COM1): ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    if input.is_empty() {
        #[cfg(not(target_os = "windows"))]
        return Ok("/dev/ttyUSB0".to_string());
        #[cfg(target_os = "windows")]
        return Ok("COM1".to_string());
    }

    Ok(input)
}

fn prompt_baudrate() -> Result<u32, Box<dyn std::error::Error>> {
    use std::io::Write;

    print!("Enter baudrate (default: 115200): ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    if input.is_empty() {
        return Ok(115200);
    }

    let baudrate: u32 = input.parse()?;
    Ok(baudrate)
}

async fn run_loop(serial: &mut SerialStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin = io::stdin();
    let mut stdin_buf = [0u8; 1024];
    let mut serial_buf = [0u8; 1024];

    loop {
        tokio::select! {
            result = serial.read(&mut serial_buf) => {
                match result {
                    Ok(n) if n > 0 => {
                        let text = String::from_utf8_lossy(&serial_buf[..n]);
                        let mut out = tokio::io::stdout();
                        out.write_all(format!("{}", text).as_bytes()).await?;
                        out.flush().await?;
                    }
                    Ok(_) => {
                        io::stdout().flush().await?;
                    } // no bytes read
                    Err(e) => {
                        eprintln!("Serial read error: {}", e);
                        break Err(Box::new(e));
                    }
                }
            }

            n = stdin.read(&mut stdin_buf) => {
                match n {
                    Ok(0) => {
                        break Ok(());
                    }
                    Ok(n) => {
                        if stdin_buf[0] == 17 {
                            println!("\r\nExiting on ctrl+q\r");
                            break Ok(());
                        }

                        serial.write_all(&stdin_buf[..n]).await?;
                    }
                    Err(e) => {
                        eprintln!("Stdin read error: {}", e);
                        break Err(Box::new(e));
                    }
                }
            }
        }
    }
}
