use crate::prelude::*; // Error , TypeErreur, Echec, Failed
                       //
use std::io::{self, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod terminal;
use terminal::*;

mod screen;
use screen::*;

mod types;
use types::*;

use super::Table;
use termion::color;

pub struct TableModifier {
    table: Vec<u8>, // table
    size: TableInfo,
    quit: bool,
    cursor: Position,
    terminal: Terminal,
    screen: ScreenBuf,
}

impl TableModifier {
    pub fn new(
        cell_size: u8,
        col: u16,
        line: u16,
        posx: u16,
        posy: u16,
    ) -> Result<Self, io::Error> {
        let table = vec![0; (cell_size as u16 * col * line) as usize];
        let terminal = Terminal::default().expect("Failed to init terminal");
        let screen = ScreenBuf::init(&terminal);
        Ok(Self {
            table,
            size: TableInfo::new(col, line, cell_size, posx, posy),
            quit: false,
            cursor: Position::new(posx + 1 + cell_size as u16 * 2, posy + 2),
            terminal,
            screen,
        })
    }

    pub fn from_table(table: Table) -> Result<Self, io::Error> {
        let terminal = Terminal::default().expect("Failed to init terminal");
        let screen = ScreenBuf::init(&terminal);
        Ok(Self {
            table: table.table,
            size: TableInfo::new(table.cols, table.lines, table.cell_size, 5, 5),
            quit: false,
            cursor: Position::new(6 + table.cell_size as u16 * 2, 7),
            terminal,
            screen,
        })
    }

    pub fn run(mut self) -> Result<Table, io::Error> {
        let _stdout = stdout().into_raw_mode().unwrap();
        self.set_table();
        loop {
            // on each keypress - check if quit, refresh
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.quit {
                println!("This is the End GOOD BYE!!!\r");
                break;
            } else {
                self.screen.draw(&self.terminal);
                //self.draw_table();
                self.update_position();
            }

            io::stdout().flush()?;

            if let Err(error) = self.proccess_keypress() {
                die(error);
            }
        }
        Ok(Table {
            table: self.table,
            cell_size: self.size.cell_size,
            cols: self.size.column,
            lines: self.size.lines,
        })
    }

    fn proccess_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.quit = true,
            Key::Char('k') => self.move_up(),
            Key::Char('j') => self.move_down(),
            Key::Char('l') => self.move_right(),
            Key::Char('h') => self.move_left(),
            Key::Char('i') => {
                if let Err(e) = self.insert() {
                    die(e);
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        io::stdout().flush()
    }

    fn set_table(&mut self) {
        let mut count = 0;
        let rcellsize = (self.size.csize() * 2) as usize;

        for line in 0..self.size.real_line() {
            if count % 2 == 1 {
                // contient les valeurs
                self.screen.set_line(
                    self.draw_content_line((line - 1) / 2),
                    self.size.posx(),
                    self.size.posy() + count,
                );
            } else {
                // les traits
                if line == 0 {
                    self.screen.set_line(
                        self.format_line('╔', '╗', '╦', &"═".repeat(rcellsize)),
                        self.size.posx(),
                        self.size.posy() + count,
                    );
                } else if line == self.size.lines * 2 {
                    self.screen.set_line(
                        self.format_line('╚', '╝', '╩', &"═".repeat(rcellsize)),
                        self.size.posx(),
                        self.size.posy() + count,
                    )
                } else {
                    self.screen.set_line(
                        self.format_line('╠', '╣', '╬', &"═".repeat(rcellsize)),
                        self.size.posx(),
                        self.size.posy() + count,
                    );
                }
            }
            count += 1;
        }
    }

    fn draw_content_line(&self, line: u16) -> String {
        let mut s: String = String::from("║"); // start
        let mut index = self.size.index_from_line(line);
        for _ in 0..self.size.column {
            for i in 0..self.size.cell_size {
                s.push_str(&format!("{:02x}", self.table[index + i as usize]));
            }
            s.push_str("║");
            index += self.size.cell_size as usize;
        }
        return s;
    }

    fn format_line(&self, start: char, end: char, sep: char, fill: &str) -> String {
        let mut fill_sep = String::from(fill);
        fill_sep.push(sep);
        format!(
            "{}{}{}{}",
            start,
            fill_sep.repeat(self.size.column as usize - 1),
            fill,
            end
        )
    }

    fn re_set_table(&mut self) {
        let mut count = 0;
        let rcellsize = (self.size.csize() * 2) as usize;

        for line in 0..self.size.real_line() {
            if count % 2 == 1 {
                // contient les valeurs
                self.screen.set_line(
                    self.draw_content_line((line - 1) / 2),
                    self.size.posx(),
                    self.size.posy() + count,
                );
            }
            count += 1;
        }
    }

    fn update_position(&self) {
        print!("{}", termion::cursor::Goto(self.cursor.x, self.cursor.y));
    }

    fn move_up(&mut self) {
        if self.cursor.y > *self.size.posy_raw() + 2 {
            self.cursor.y -= 2
        }
    }

    fn move_down(&mut self) {
        if self.cursor.y < (self.size.lines - 1) * 2 + 2 + self.size.posy_raw() {
            self.cursor.y += 2;
        }
    }

    fn move_left(&mut self) {
        if self.cursor.x > self.size.cell_size as u16 * 2 + self.size.posx_raw() + 1 {
            self.cursor.x -= self.size.cell_size as u16 * 2 + 1
        }
    }

    fn move_right(&mut self) {
        if self.cursor.x
            < (self.size.column) * (self.size.cell_size as u16 * 2 + 1) + self.size.posx_raw()
        {
            self.cursor.x += self.size.cell_size as u16 * 2 + 1
        }
    }

    fn cursor_to_index(&self) -> usize {
        let csize = self.size.cell_size as u16;

        let nb_line = ((self.cursor.y - self.size.posy_raw()) / 2) - 1;
        let nb_col = ((self.cursor.x - self.size.posx_raw()) / (csize as u16 * 2 + 1)) - 1;
        // we remove one to be zero based
        (nb_line * self.size.column * csize + nb_col * csize) as usize
    }

    fn insert(&mut self) -> Result<(), std::io::Error> {
        let mut count: usize = 0;
        let index = self.cursor_to_index();
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            self.screen.draw(&self.terminal);
            //self.draw_table();
            self.update_position();
            io::stdout().flush()?;

            let pressed_key = read_key()?;
            match pressed_key {
                Key::Esc | Key::Ctrl('c') => break,
                Key::Char(c) => {
                    if c >= '0' && c <= '9' || c >= 'a' && c <= 'f' {
                        let val = c.to_digit(16).unwrap() as u8;
                        if count % 2 == 0 {
                            self.table[index + (count / 2)] = val;
                        } else {
                            self.table[index + (count - 1) / 2] &= 15;
                            self.table[index + (count - 1) / 2] += val << 4;
                        }
                        count += 1;
                    }
                }
                _ => continue,
            }

            self.re_set_table();
            if count + 1 > self.size.cell_size as usize * 2 {
                break;
            }
        }
        Ok(())
    }
}

pub fn read_key() -> Result<Key, io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}
