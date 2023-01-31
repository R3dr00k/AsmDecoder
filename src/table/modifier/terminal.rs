use super::types::*;

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let tsize = termion::terminal_size()?;
        Ok(Self {
            size: Size::new(tsize.0, tsize.1),
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
