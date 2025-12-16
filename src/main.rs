use crossterm::{ 
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode, size},
    queue,
    event::{self, KeyCode, Event},
    cursor::{MoveTo},
};
use std::{
    io::{self, Write}, 
    fs::{self},
    error::{Error},
    path::{Path},
};

enum ElementType {
    Return,
    File,
    Directory,
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
    name: String,
    element_type: ElementType,
}
impl Element {
    pub fn new(new_name: String, new_element_type: ElementType) -> Self {
        Element{ name: new_name, element_type: new_element_type }
    }
}
struct Dir {
    count: usize,
    elements: Vec<Element>,
    path: String,
    parent_path: String,
}
impl Dir {
    pub fn new() -> Self {
        Dir{ count: 0, elements: Vec::new(), path: String::from("/") , parent_path: String::new() }
    }
    pub fn push_element(&mut self, element: Element) {
        self.elements.push(element);
        self.count += 1;
    }
    pub fn clear_elements(&mut self) {
        self.elements.clear();
    }
}

fn get_elements_from_dir(dir: &mut Dir, global: &mut Global) -> Result<(), Box<dyn Error>> {
    dir.clear_elements();

    let path_name = &dir.path;
    let path = Path::new(path_name);
    let parent = path.parent();
    match parent {
        Some(p) => dir.parent_path = p.to_string_lossy().to_string(),
        None => dir.parent_path = String::from("/"),
    }
    dir.push_element(Element::new(String::from(format!("{}", dir.path)), ElementType::File));
    dir.push_element(Element::new(String::from("(Back)"), ElementType::Return));
    for element in fs::read_dir(&dir.path)? {
        let element = element?;
        let file_type = if element.file_type()?.is_dir() {
                ElementType::Directory
            }else if element.file_type()?.is_file() {
                ElementType::File
            }else {
                ElementType::File
            };
        dir.push_element(Element::new(element.file_name().into_string().unwrap(), file_type));
    }
    dir.count = dir.elements.len();
    if dir.count < global.index {
        global.index = dir.count - 1;
    }
    Ok(())
}

fn print_elements(dir: &Dir, global: &Global) -> io::Result<()> {
    let mut stdout = io::stdout();
    for (i, element) in dir.elements.iter().enumerate() {
        let _ = queue!(stdout, MoveTo(0, i as u16));
        let name = &element.name;
        if i == global.index {
            let selected_name = format!("{name} <-");
            stdout.write(selected_name.as_bytes())?;
        }else {
            stdout.write(name.as_bytes())?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut dir = Dir::new();
    let mut global = Global::new();
    let mut stdout = io::stdout();
    loop{
        let _ = get_elements_from_dir(&mut dir, &mut global);
        let _ = queue!(stdout, Clear(ClearType::All));
        print_elements(&dir, &global)?;
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
                    //go back
                    if global.index < 2 {
                        if dir.path.len() > 1 {
                            dir.path = dir.parent_path.clone();
                        }
                        else {
                            continue;
                        }
                    }
                    else {
                        let dir_name = &dir.elements[global.index].name;
                        if dir.path.len() > 1 {
                            dir.path.push_str(format!("/{}", dir_name).as_str());
                        }else {
                            dir.path.push_str(format!("{}", dir_name).as_str());
                        }
                    }
                }
                _ => {},
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
