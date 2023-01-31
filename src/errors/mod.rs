use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Echec {
    source: TypeErreur,
    fichier: String,
    ligne: u32,
}

impl Echec {
    pub fn new(source: TypeErreur, fichier: &str, ligne: u32) -> Self {
        Echec {
            source,
            fichier: fichier.to_string(),
            ligne,
        }
    }
}
impl fmt::Display for Echec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Echec happened in {} at line {}",
            self.fichier, self.ligne
        )
    }
}

impl Error for Echec {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
pub enum TypeErreur {
    InvalidArraySize,
    InvalidSiveV,
}

impl Error for TypeErreur {}

impl fmt::Display for TypeErreur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeErreur::InvalidArraySize => {
                return write!(f, "Array indexing error when reading disp bytes")
            }
            TypeErreur::InvalidSiveV => return write!(f, "SizeV can only read : 1,2,3 or 4 bytes"),
        }
    }
}

#[macro_export]
macro_rules! failed {
    ($arg:expr) => {
        Err(Box::new(Echec::new($arg, file!(), line!())))
    };
}

pub fn die<T: Error>(e: T) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
