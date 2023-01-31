use super::SizeV;

// ================ Operand Trait =====================
pub trait Operand {}

pub struct Reg {
    value: u8,
    size: u8,
}

impl Operand for Reg {}
impl Reg {
    pub fn new(value: u8, size: u8) -> Self {
        Self { value, size }
    }
}

pub struct Rm {
    rm: u8,
    sib: Option<Sib>,
    disp: Option<SizeV>,
    size: u8,
}

pub struct Sib {
    scale: u8,
    base: u8,
    index: u8,
}

impl Sib {
    pub fn new(x: u8) -> Self {
        Self {
            scale: x >> 6,
            index: (x & 0x38) >> 3,
            base: x & 7,
        }
    }
}

impl Operand for Rm {}
impl Rm {
    pub fn new(rm: u8, sib: Option<Sib>, disp: Option<SizeV>, size: u8) -> Self {
        Self {
            rm,
            sib,
            disp,
            size,
        }
    }
}
pub struct Imm(pub SizeV);
impl Operand for Imm {}
