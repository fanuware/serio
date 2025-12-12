use std::io::{self, Write};

pub fn prompt_device() -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(not(target_os = "windows"))]
    print!("Enter serial device (default: /dev/ttyUSB0): ");

    #[cfg(target_os = "windows")]
    print!("Enter serial device (default: COM1): ");

    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    if input.is_empty() {
        #[cfg(not(target_os = "windows"))]
        return Ok("/dev/ttyUSB0".to_string());

        #[cfg(target_os = "windows")]
        return Ok("COM1".to_string());
    }

    Ok(input)
}

pub fn prompt_baudrate() -> Result<u32, Box<dyn std::error::Error>> {
    print!("Enter baudrate (default: 115200): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim();
    if input.is_empty() {
        return Ok(115200);
    }

    Ok(input.parse()?)
}
