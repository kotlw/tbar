use zellij_tile::prelude::*;

#[derive(Debug)]
pub enum Style {
    Bg(Color),
    Fg(Color),
    Bold,
    Default,
}

#[derive(Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Orange,
    Gray,
    Purple,
    Gold,
    Silver,
    Pink,
    Brown,
}

fn get_color(palette: &Palette, color: &Color) -> ansi_term::Color {
    let p = match color {
        Color::Black => palette.black,
        Color::Red => palette.red,
        Color::Green => palette.green,
        Color::Yellow => palette.yellow,
        Color::Blue => palette.blue,
        Color::Magenta => palette.magenta,
        Color::Cyan => palette.cyan,
        Color::White => palette.white,
        Color::Orange => palette.orange,
        Color::Gray => palette.gray,
        Color::Purple => palette.purple,
        Color::Gold => palette.gold,
        Color::Silver => palette.silver,
        Color::Pink => palette.pink,
        Color::Brown => palette.brown,
    };

    match p {
        PaletteColor::Rgb((r, g, b)) => ansi_term::Color::RGB(r, g, b),
        PaletteColor::EightBit(color) => ansi_term::Color::Fixed(color),
    }
}

pub fn render(palette: &Palette, style: &Style) -> String {
    let s = ansi_term::Style::new();
    match style {
        Style::Fg(c) => s.fg(get_color(palette, c)).prefix().to_string(),
        Style::Bg(c) => s.on(get_color(palette, c)).prefix().to_string(),
        Style::Bold => s.bold().prefix().to_string(),
        Style::Default => s.on(ansi_term::Color::Fixed(0)).suffix().to_string(),
    }
}

#[derive(Default)]
pub struct StyleRenderer {
    palette: Palette,
}

impl StyleRenderer {
    fn get_color(&self, c: &Color) -> ansi_term::Color {
        let p = match c {
            Color::Black => self.palette.black,
            Color::Red => self.palette.red,
            Color::Green => self.palette.green,
            Color::Yellow => self.palette.yellow,
            Color::Blue => self.palette.blue,
            Color::Magenta => self.palette.magenta,
            Color::Cyan => self.palette.cyan,
            Color::White => self.palette.white,
            Color::Orange => self.palette.orange,
            Color::Gray => self.palette.gray,
            Color::Purple => self.palette.purple,
            Color::Gold => self.palette.gold,
            Color::Silver => self.palette.silver,
            Color::Pink => self.palette.pink,
            Color::Brown => self.palette.brown,
        };

        match p {
            PaletteColor::Rgb((r, g, b)) => ansi_term::Color::RGB(r, g, b),
            PaletteColor::EightBit(color) => ansi_term::Color::Fixed(color),
        }
    }

    pub fn update(&mut self, palette: Palette) -> bool {
        if self.palette != palette {
            self.palette = palette;
            return true;
        }
        false
    }

    pub fn render(&self, style: &Style) -> String {
        let s = ansi_term::Style::new();
        match style {
            Style::Fg(c) => s.fg(self.get_color(c)).prefix().to_string(),
            Style::Bg(c) => s.on(self.get_color(c)).prefix().to_string(),
            Style::Bold => s.bold().prefix().to_string(),
            Style::Default => s.on(ansi_term::Color::Fixed(0)).suffix().to_string(),
        }
    }
}
