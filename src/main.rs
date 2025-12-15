use crossterm::{ 
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode, size},
    queue,
    event::{self, KeyCode, Event},
    cursor::{MoveTo},
};
use std::{
    io::{self, Write}, 
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let label = b"Hello, World!";
    loop{
        let _ = queue!(stdout, Clear(ClearType::All));
        let (w, h) = size()?;
        let _ = queue!(stdout, MoveTo( w / 2 - label.len() as u16 / 2, h / 2));
        stdout.write(label)?;
        stdout.flush()?;
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
