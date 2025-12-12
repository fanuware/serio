use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct RawModeGuard;

impl RawModeGuard {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        Ok(RawModeGuard)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

pub fn key_to_bytes(code: KeyCode, kind: KeyEventKind, modifiers: KeyModifiers) -> Option<Vec<u8>> {
    if kind != KeyEventKind::Press {
        return None;
    }

    let bytes = match (code, modifiers) {
        (KeyCode::Up, KeyModifiers::CONTROL) => vec![0x1B, 0x5B, 0x31, 0x3B, 0x35, 0x41],
        (KeyCode::Up, _) => vec![0x1B, 0x5B, 0x41],
        (KeyCode::Down, KeyModifiers::CONTROL) => vec![0x1B, 0x5B, 0x31, 0x3B, 0x35, 0x42],
        (KeyCode::Down, _) => vec![0x1B, 0x5B, 0x42],
        (KeyCode::Left, KeyModifiers::CONTROL) => vec![0x1B, 0x5B, 0x31, 0x3B, 0x35, 0x44],
        (KeyCode::Left, _) => vec![0x1B, 0x5B, 0x44],
        (KeyCode::Right, KeyModifiers::CONTROL) => vec![0x1B, 0x5B, 0x31, 0x3B, 0x35, 0x43],
        (KeyCode::Right, _) => vec![0x1B, 0x5B, 0x43],
        (KeyCode::Home, KeyModifiers::CONTROL) => vec![0x1B, 0x5B, 0x31, 0x3B, 0x35, 0x48],
        (KeyCode::Home, _) => vec![0x1B, 0x5B, 0x48],
        (KeyCode::End, KeyModifiers::CONTROL) => vec![0x1B, 0x5B, 0x31, 0x3B, 0x35, 0x46],
        (KeyCode::End, _) => vec![0x1B, 0x5B, 0x46],
        (KeyCode::PageUp, KeyModifiers::CONTROL) => vec![0x1B, 0x5B, 0x35, 0x3B, 0x35, 0x7E],
        (KeyCode::PageUp, _) => vec![0x1B, 0x5B, 0x35, 0x7E],
        (KeyCode::PageDown, KeyModifiers::CONTROL) => vec![0x1B, 0x5B, 0x36, 0x3B, 0x35, 0x7E],
        (KeyCode::PageDown, _) => vec![0x1B, 0x5B, 0x36, 0x7E],
        (KeyCode::Tab, _) => vec![0x09],
        (KeyCode::BackTab, _) => vec![0x1B, 0x5B, 0x5A],
        (KeyCode::Delete, KeyModifiers::CONTROL) => vec![0x1B, 0x5B, 0x33, 0x3B, 0x35, 0x7E],
        (KeyCode::Delete, _) => vec![0x1B, 0x5B, 0x33, 0x7E],
        (KeyCode::Insert, KeyModifiers::CONTROL) => vec![0x1B, 0x5B, 0x32, 0x3B, 0x35, 0x7E],
        (KeyCode::Insert, _) => vec![0x1B, 0x5B, 0x32, 0x7E],
        (KeyCode::Esc, _) => vec![0x1B],
        (KeyCode::Backspace, _) => vec![0x7F],
        (KeyCode::Enter, _) => vec![0x0D],
        (KeyCode::Char(c), KeyModifiers::CONTROL) if c.is_ascii() => vec![c as u8 & 0x1F],
        (KeyCode::Char(c), _) => c.encode_utf8(&mut [0; 4]).as_bytes().to_vec(),
        (KeyCode::F(n), _) => {
            let f_key = n + 11;
            vec![0x1B, 0x5B, f_key, 0x7E]
        }
        _ => return None,
    };

    Some(bytes)
}
