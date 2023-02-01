mod decoder;
use decoder::decode;

mod types;
use types::*;

mod table;
use table::*;

mod errors;

mod prelude {
    pub use crate::errors::{die, Echec, TypeErreur};
    pub use crate::failed;
    pub use std::error::Error;
}

use prelude::*;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().unwrap();

    match args.next() {
        Some(x) => match x.as_str() {
            "--modify-table" => match args.next() {
                Some(filename) => {
                    if let Err(e) = Table::modify(&filename) {
                        die(e);
                    }
                }
                None => {
                    help(&progname);
                }
            },
            _ => (),
        },
        None => {
            // here normal execution
            help(&progname);
        }
    }

    let x = match Tables::init() {
        Ok(x) => x,
        Err(e) => die(e),
    };
}

fn help(name: &str) {
    println!("usage ./{} <args>|<args value>", name);
    print!(
        "List of the possibles arguments:\n\r\
                --modify-table <tablename> : open the terminal gui to modify table\n\r\
           "
    );
}
