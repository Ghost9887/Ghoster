use crossterm::{ 
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode, size},
    queue,
    event::{self, KeyCode, Event},
    cursor::{MoveTo},
};
use std::{
    io::{self, Write}, 
};

enum Element_Type {
    File,
    Directory,
    HiddenFile,
}

struct Element {
    name: String,
    elemnt_type: Element_Type,
}
struct Dir {
    count: usize,
    elements: Vec<Element>,
}
impl Dir {
    pub fn new() -> Self {
        Dir{ count: 0, elements: Vec::new() }
    }
    pub fn push_element(&mut self, element: Element) {
        self.elements.push(element);
        self.count += 1;
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    loop{
        let _ = queue!(stdout, Clear(ClearType::All));
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
