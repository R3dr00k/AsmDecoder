// load_file -> Mmap good size (when table is ready for usage)
//
// create or modify table -> alloc to heap
// apply modif(termion) -> write file -> free allocation
mod modifier;
pub use modifier::*;
use std::fs::{self};
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

use crate::prelude::*; // Error , TypeErreur, Echec, Failed

static TABLES_DIR: &'static str = "/home/me/.local/share/marcus"; // a completer

pub struct Table {
    table: Vec<u8>,
    cell_size: u8,
    cols: u16,
    lines: u16,
}

pub struct Tables {
    tables: Vec<Table>,
}

impl Tables {
    pub fn init() -> Result<Self, io::Error> {
        // check if dir exist & or create it
        if !Path::new(TABLES_DIR).is_dir() {
            fs::create_dir(TABLES_DIR)?;
        }

        let mut vec = Vec::new();
        let paths = fs::read_dir(TABLES_DIR).unwrap();
        for file in paths {
            match Table::load(file.unwrap().path()) {
                Ok(x) => vec.push(x),
                Err(e) => die(e),
            }
        }

        Ok(Self { tables: vec })
    }
}

impl Table {
    pub fn load(path: PathBuf) -> Result<Self, io::Error> {
        // take dir load each file in this dir to
        let mut bytes = fs::read(path)?;
        let cell_size = bytes.pop().unwrap();
        let cols: u16 = ((bytes.pop().unwrap() as u16) << 8_u8) + (bytes.pop().unwrap()) as u16;
        let lines: u16 = ((bytes.pop().unwrap() as u16) << 8_u8) + (bytes.pop().unwrap()) as u16;

        if cell_size as u16 * cols * lines != bytes.len() as u16 {
            return Err(io::Error::new(ErrorKind::Other, "invalid table size"));
        }

        Ok(Self {
            table: bytes,
            cell_size,
            cols,
            lines,
        })
    }

    pub fn modify(file: &str) -> Result<(), io::Error> {
        if !Path::new(TABLES_DIR).is_dir() {
            fs::create_dir(TABLES_DIR)?;
        }
        let mut path = PathBuf::from(TABLES_DIR);
        path.push(file);
        let table = Table::load(path)?;

        let mo = TableModifier::from_table(table)?;

        let table = match mo.run() {
            Ok(x) => x,
            Err(e) => die(e),
        };
        Table::save(table, file)?;
        Ok(())
    }

    pub fn create() -> Result<(), io::Error> {
        if !Path::new(TABLES_DIR).is_dir() {
            fs::create_dir(TABLES_DIR)?;
        }
        let mo = TableModifier::new(4, 0xF, 0xF, 1, 1).unwrap();

        let table = match mo.run() {
            Ok(x) => x,
            Err(e) => die(e),
        };
        if let Err(e) = Table::save(table, &format!("table{}", nth_of_file())) {
            die(e);
        }
        Ok(())
        // take a file (table) -> load it in vec on heap
        // init TableModifier
    }

    fn save(table: Table, filename: &str) -> Result<(), io::Error> {
        let mut path = PathBuf::from(TABLES_DIR);
        path.push(filename);
        let mut bytes = table.table;
        bytes.push((table.cols & 15) as u8);
        bytes.push((table.cols >> 8) as u8);
        bytes.push((table.lines & 15) as u8);
        bytes.push((table.lines >> 8) as u8);
        bytes.push(table.cell_size);
        fs::write(path, bytes)?;

        Ok(())
    }
}

fn nth_of_file() -> usize {
    fs::read_dir(TABLES_DIR).iter().count()
}
