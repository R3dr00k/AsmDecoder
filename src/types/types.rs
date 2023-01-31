use crate::prelude::*; // Error , TypeErreur, Echec, failed

#[derive(Clone)]
pub enum SizeV {
    Byte(u8),
    Word(u16),
    Dword(u32),
    Qword(u64),
}

impl SizeV {
    pub fn from_fill(size: u8, bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let len = bytes.len();
        match size {
            1 => {
                if len < 1 {
                    return failed!(TypeErreur::InvalidArraySize);
                }
                return Ok(Self::Byte(bytes[0]));
            }
            2 => {
                if len < 2 {
                    return failed!(TypeErreur::InvalidArraySize);
                }
                return Ok(Self::Word(as_u16_le(bytes)));
            }
            3 => {
                if len < 4 {
                    return failed!(TypeErreur::InvalidArraySize);
                }
                return Ok(Self::Dword(as_u32_le(bytes)));
            }
            4 => {
                if len < 8 {
                    return failed!(TypeErreur::InvalidArraySize);
                }
                return Ok(Self::Qword((as_u64_le(bytes))));
            }
            _ => {
                return failed!(TypeErreur::InvalidSiveV);
            }
        }
    }

    pub fn fill(&mut self, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        let len = bytes.len();
        match self {
            Self::Byte(mut x) => {
                if len < 1 {
                    return failed!(TypeErreur::InvalidArraySize);
                }
                x = bytes[0];
            }
            Self::Word(mut x) => {
                if len < 2 {
                    return failed!(TypeErreur::InvalidArraySize);
                }
                x = as_u16_le(bytes);
            }
            Self::Dword(mut x) => {
                if len < 4 {
                    return failed!(TypeErreur::InvalidArraySize);
                }
                x = as_u32_le(bytes);
            }
            Self::Qword(mut x) => {
                if len < 8 {
                    return failed!(TypeErreur::InvalidArraySize);
                }
                x = as_u64_le(bytes);
            }
        }
        Ok(())
    }

    pub fn nb_bytes(&self) -> u8 {
        match self {
            Self::Byte(_) => 1,
            Self::Word(_) => 2,
            Self::Dword(_) => 4,
            Self::Qword(_) => 8,
        }
    }
}

fn as_u32_le(array: &[u8]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}

fn as_u16_le(array: &[u8]) -> u16 {
    (array[0] as u16) << 8 + (array[1] as u16)
}

fn as_u64_le(array: &[u8]) -> u64 {
    ((array[0] as u64) << 0)
        + ((array[1] as u64) << 8)
        + ((array[2] as u64) << 16)
        + ((array[3] as u64) << 24)
        + ((array[4] as u64) << 32)
        + ((array[5] as u64) << 40)
        + ((array[6] as u64) << 48)
        + ((array[7] as u64) << 56)
}
