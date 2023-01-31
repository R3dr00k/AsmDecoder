// ================================================

#[derive(Copy, Clone)]
pub struct Size {
    width: u16,
    height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }
}

// ================================================

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}

// ================================================

pub struct TableInfo {
    pub column: u16,
    pub lines: u16,
    pub cell_size: u8,
    position: Position,
}

impl TableInfo {
    pub fn new(column: u16, lines: u16, cell_size: u8, x: u16, y: u16) -> Self {
        Self {
            column,
            lines,
            cell_size,
            position: Position::new(x, y),
        }
    }

    pub fn csize(&self) -> &u8 {
        &self.cell_size
    }

    pub fn real_line(&self) -> u16 {
        (self.lines * 2) + 1
    }

    pub fn index_from_line(&self, line: u16) -> usize {
        (line * self.column * self.cell_size as u16) as usize
    }

    pub fn posx(&self) -> usize {
        self.position.x as usize
    }

    pub fn posy(&self) -> usize {
        self.position.y as usize
    }

    pub fn posx_raw(&self) -> &u16 {
        &self.position.x
    }

    pub fn posy_raw(&self) -> &u16 {
        &self.position.y
    }
}
