use super::terminal::Terminal;
use super::types::*;
use termion::color;

pub struct ScreenBuf {
    buffer: Vec<Vec<Pixel>>,
    size: Size,
}

#[derive(Clone)]
struct Pixel {
    val: char,
    color_fg: Option<String>,
    color_bg: Option<String>,
}

impl Pixel {
    pub fn new(x: char) -> Self {
        Self {
            val: x,
            color_fg: None,
            color_bg: None,
        }
    }

    pub fn draw(&self) {
        let mut need_to_reset_fg = false;
        let mut need_to_reset_bg = false;
        match &self.color_fg {
            Some(x) => {
                print!("{}", x);
                need_to_reset_fg = true;
            }
            None => (),
        }
        match &self.color_bg {
            Some(x) => {
                print!("{}", x);
                need_to_reset_bg = true;
            }
            None => (),
        }
        print!("{}", self.val);
        if need_to_reset_fg {
            print!("{}", color::Fg(color::Reset));
        }
        if need_to_reset_bg {
            print!("{}", color::Bg(color::Reset));
        }
    }
}

impl ScreenBuf {
    pub fn init(term: &Terminal) -> Self {
        let size = term.size().clone();

        let buf =
            vec![vec![Pixel::new(' '); size.get_width() as usize]; size.get_height() as usize];

        Self { buffer: buf, size }
    }

    pub fn draw(&self, term: &Terminal) {
        let size = term.size();
        for line in 0..size.get_height() as usize {
            print!("\r\n");
            for c in 0..size.get_width() as usize {
                self.buffer[line][c].draw();
            }
        }
    }

    pub fn set(&mut self, c: char, x: usize, y: usize) {
        self.buffer[y][x].val = c;
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

    pub fn change_color(&mut self, x: usize, y: usize, c_fg: (u8, u8, u8), c_bg: (u8, u8, u8)) {
        self.buffer[y][x].color_fg =
            Some(color::AnsiValue::rgb(c_fg.0, c_fg.1, c_fg.2).fg_string());
        self.buffer[y][x].color_bg =
            Some(color::AnsiValue::rgb(c_bg.0, c_bg.1, c_bg.2).bg_string());
    }
}
