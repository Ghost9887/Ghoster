use crossterm::{ 
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode, size},
    queue,
    event::{self, KeyCode, Event},
    cursor::{MoveTo, MoveUp, MoveDown},
};
use std::{
    io::{self, Write}, 
    fs::{self},
    error::{Error},
    ffi::{OsString},
};

enum ElementType {
    File,
    Directory,
    HiddenFile,
}

enum Mode {
    Navigating,
}

struct Global {
    index: usize,
    t_height: u16,
    t_width: u16,
    mode: Mode,
}
impl Global {
    pub fn new() -> Self {
        let (t_width, t_height) = size().unwrap();
        Global { 
            index: 0, 
            t_height: t_height, 
            t_width: t_width, 
            mode: Mode::Navigating, 
        }
    }
}

struct Element {
    name: OsString,
    element_type: ElementType,
}
impl Element {
    pub fn new(new_name: OsString, new_element_type: ElementType) -> Self {
        Element{ name: new_name, element_type: new_element_type }
    }
}
struct Dir {
    count: usize,
    elements: Vec<Element>,
    //path: String,
}
impl Dir {
    pub fn new() -> Self {
        Dir{ count: 0, elements: Vec::new()/*, path: String::from(".")*/ }
    }
    pub fn push_element(&mut self, element: Element) {
        self.elements.push(element);
        self.count += 1;
    }
}

fn get_elements_from_dir(path: &str, dir: &mut Dir) -> Result<(), Box<dyn Error>> {
    for element in fs::read_dir(path)? {
        let element = element?;
        let file_type = if element.file_type()?.is_dir() {
                ElementType::Directory
            }else if element.file_type()?.is_file() {
                ElementType::File
            }else {
                ElementType::File
            };
        dir.push_element(Element::new(element.file_name(), file_type));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut dir = Dir::new();
    let mut global = Global::new();
    let _ = get_elements_from_dir(".", &mut dir);
    let mut stdout = io::stdout();
    let mut shown = false;
    loop{
        let _ = queue!(stdout, Clear(ClearType::All));
        for (i, element) in dir.elements.iter().enumerate() {
            let _ = queue!(stdout, MoveTo(0, i as u16));
            let name = element.name.to_string_lossy();
            if i == global.index {
                let selected_name = format!("{name} <-");
                stdout.write(selected_name.as_bytes())?;
            }else {
                stdout.write(name.as_bytes())?;
            }
        }
        let _ = queue!(stdout, MoveTo(0, global.t_height - 2));
        stdout.write(("─".repeat(global.t_width as usize)).as_bytes())?;
        let _ = queue!(stdout, MoveTo(0, global.t_height));
        stdout.flush()?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('j') => {
                    if global.index < dir.elements.len() - 1 {
                        global.index += 1;
                    }
                }
                KeyCode::Char('k') => {
                    if global.index > 0 {
                        global.index -= 1;
                    }
                }
                KeyCode::Enter => {
                    todo!();
                }
                _ => {},
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
