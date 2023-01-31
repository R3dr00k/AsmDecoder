mod types;
use types::SizeV;

mod instruction;
pub use instruction::Inst;

mod operands;
use operands::{Imm, Reg, Rm, Sib};

use crate::prelude::*; // Error , TypeErreur, Echec
                       //
pub enum OpTypes {
    Modrm(u8), // higher bit def sens 3th first byte def size of reg , the next 2 bits are size of
    // rm
    Imm(SizeV),
    Reg(u8, u8), // for implicit parameter
    Moffs(SizeV),
    //Addr(),
    MemOnly, // modrm but mod != 11
}

impl Inst {
    fn parser(&mut self, ops: Vec<OpTypes>, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        // consider that index 0 is the next byte after opcode
        let mut index = 0;

        for kind in ops {
            match kind {
                OpTypes::Modrm(x) => {
                    // x hb=1 -> reg, r/m, hb=0 -> r/m reg
                    let mode = bytes[index] >> 6;
                    let reg = (bytes[index] & 0x38) >> 3;
                    let rm = bytes[index] & 7;

                    index += 1;

                    let mut sib: Option<Sib> = None;
                    let mut disp: Option<SizeV> = None;

                    if mode == 3 {
                        // reg to reg
                        if x & 0x80 == 1 {
                            self.push(Box::new(Reg::new(reg, x & 7)));
                            self.push(Box::new(Reg::new(rm, (x & 24) >> 3)));
                        } else {
                            self.push(Box::new(Reg::new(rm, (x & 24) >> 3)));
                            self.push(Box::new(Reg::new(reg, x & 7)));
                        }
                    } else {
                        if rm == 4 {
                            sib = Some(Sib::new(bytes[index + 1]));
                            index += 1;
                        }

                        if mode == 2 {
                            // base + disp32
                            disp = Some(SizeV::from_fill(3, &bytes[index..])?);
                            index += 4;
                        } else if mode == 1 {
                            // base + disp8
                            disp = Some(SizeV::from_fill(1, &bytes[index..])?);
                            index += 1;
                        }

                        if x & 0x80 == 1 {
                            self.push(Box::new(Reg::new(reg, x & 7)));
                            self.push(Box::new(Rm::new(rm, sib, disp, (x & 24) >> 3)));
                        } else {
                            self.push(Box::new(Rm::new(rm, sib, disp, (x & 24) >> 3)));
                            self.push(Box::new(Reg::new(reg, x & 7)));
                        }
                    }
                }
                OpTypes::Imm(mut x) => {
                    // hell no
                    x.fill(&bytes)?;
                    index += x.nb_bytes() as usize;
                    self.push(Box::new(Imm(x)));
                }
                OpTypes::Reg(x, y) => self.push(Box::new(Reg::new(x, y))),
                _ => (),
            }
        }

        return Ok(());
    }
}
