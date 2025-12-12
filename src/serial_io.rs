use super::terminal::key_to_bytes;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialStream;

pub async fn run_loop(serial: &mut SerialStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut serial_buf = [0u8; 1024];

    let mut stdin_reader = EventStream::new();

    loop {
        let stdin_event = stdin_reader.next();

        tokio::select! {
            res = serial.read(&mut serial_buf) => {
                match res {
                    Ok(n) if n > 0 => {
                        let text = String::from_utf8_lossy(&serial_buf[..n]);
                        let mut out = tokio::io::stdout();
                        out.write_all(text.as_bytes()).await?;
                        out.flush().await?;
                    }
                    Ok(_) => {
                        io::stdout().flush().await?;
                    }
                    Err(e) => {
                        eprintln!("Serial read error: {}", e);
                        break Err(Box::new(e));
                    }
                }
            }

            maybe_event = stdin_event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        match event {
                            Event::Paste(text) => {
                                serial.write_all(text.as_bytes()).await?;
                            }
                            Event::Key(KeyEvent { code, kind, modifiers, .. }) => {
                                if let (KeyCode::Char('q'), KeyModifiers::CONTROL) = (code, modifiers) {
                                    println!("\r\nExiting on Ctrl+Q\r");
                                    break Ok(());
                                }

                                if kind == KeyEventKind::Press {
                                    if let Some(bytes) = key_to_bytes(code, kind, modifiers) {
                                        serial.write_all(&bytes).await?;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Some(Err(e)) => break Err(Box::new(e)),
                    None => break Ok(()), // End of stream
                }
            }
        }
    }
}
