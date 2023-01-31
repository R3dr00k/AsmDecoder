use super::terminal::Terminal;
use super::types::*;

pub struct ScreenBuf {
    buffer: Vec<Vec<char>>,
    size: Size,
}

impl ScreenBuf {
    pub fn init(term: &Terminal) -> Self {
        let size = term.size().clone();

        let buf = vec![vec![' '; size.get_width() as usize]; size.get_height() as usize];

        Self { buffer: buf, size }
    }

    pub fn draw(&self, term: &Terminal) {
        let size = term.size();
        for line in 0..size.get_height() as usize {
            print!("\r\n");
            for c in 0..size.get_width() as usize {
                print!("{}", self.buffer[line][c]);
            }
        }
    }

    pub fn set(&mut self, c: char, x: usize, y: usize) {
        self.buffer[y][x] = c;
    }

    pub fn set_line(&mut self, s: String, x: usize, y: usize) {
        let mut x = x;
        for c in s.chars() {
            if x > self.size.get_width() as usize || y > self.size.get_height() as usize {
                break;
            }
            self.set(c, x, y);
            x += 1;
        }
    }
}
