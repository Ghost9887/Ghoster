use crossterm::{ 
    terminal::{Clear, ClearType, size, enable_raw_mode, disable_raw_mode},
    queue,
    cursor::{MoveTo},
    event::{self, KeyCode, Event},
};
use std::{
    io::{self, Write}, 
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let (w, h) = size()?;
    let label = b"Hello, World!";
    let _ = queue!(stdout, Clear(ClearType::All));
    let _ = queue!(stdout, MoveTo(w / 2 - label.len() as u16 / 2, h / 2));
    stdout.write(label)?;
    stdout.flush()?;
    loop{
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {},
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
