// load_file -> Mmap good size (when table is ready for usage)
//
// create or modify table -> alloc to heap
// apply modif(termion) -> write file -> free allocation
mod modifier;
use modifier::*;

use crate::prelude::*; // Error , TypeErreur, Echec, Failed

pub struct Tables {}

impl Tables {
    pub fn load() -> Self {
        // take dir load each file in this dir to a mmap
        Tables {}
    }

    pub fn modify() {
        let mut mo = TableModifier::new(4, 0x6, 0xF, 1, 1).unwrap();
        if let Err(e) = mo.run() {
            die(e);
        }
        // take a file (table) -> load it in vec on heap
        // init TableModifier
    }
}
