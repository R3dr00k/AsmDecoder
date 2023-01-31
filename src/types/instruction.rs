use super::{operands::*, OpTypes};
use std::process;

pub struct Inst {
    name: String,
    nb: u8,
    opds: Vec<Box<dyn Operand>>,
}

impl Inst {
    pub fn from_parser(name: &str, ops: Vec<OpTypes>, bytes: &[u8]) -> Self {
        // call parser handle error , return instance of himself
        let mut ins = Inst {
            name: name.to_string(),
            nb: 0,
            opds: Vec::new(),
        };

        if let Err(e) = ins.parser(ops, bytes) {
            eprintln!("{}", e);
            process::exit(0);
        };

        return ins;
    }

    pub fn push(&mut self, val: Box<dyn Operand>) {
        self.opds.push(val);
    }

    /*pub fn push(&mut self, val: T) {
        match self.opds {
            Some(x) => x.push(val),
            None => {
                self.opds = Some(Linked {
                    value: val,
                    next: None,
                })
            }
        }
    }*/
}
