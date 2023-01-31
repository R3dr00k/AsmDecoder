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

fn main() {
    Tables::modify();
}
