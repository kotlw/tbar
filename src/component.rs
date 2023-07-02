use crate::config::Config;
use crate::parser::parse;
use ansi_term::{Color, Style as ansi_style};
use zellij_tile::prelude::*;

#[allow(unused_variables)]
pub trait Component {
    fn get(&self) -> &str;
    fn len(&self) -> usize;
    fn update(&mut self, event: &Event, config: &Config) {}
}

// Text -----------------------------------------------------------------------
#[derive(Debug)]
pub struct Text {
    value: String,
}

impl Text {
    pub fn new(value: String) -> Text {
        Text { value }
    }
}

impl Component for Text {
    fn get(&self) -> &str {
        &self.value
    }

    fn len(&self) -> usize {
        self.value.chars().count()
    }
}

// Session --------------------------------------------------------------------
#[derive(Debug, Default)]
pub struct Session {
    name: String,
}

impl Session {
    pub fn new() -> Session {
        Session {
            ..Default::default()
        }
    }
}

impl Component for Session {
    fn get(&self) -> &str {
        &self.name
    }

    fn len(&self) -> usize {
        self.name.chars().count()
    }

    fn update(&mut self, event: &Event, _config: &Config) {
        if let Event::ModeUpdate(mode_info) = event {
            if let Some(session_name) = &mode_info.session_name {
                self.name = session_name.to_string();
            }
        }
    }
}

// Style ----------------------------------------------------------------------
#[derive(Debug)]
pub struct Style {
    value: String,
    color_code: String,
}

impl Style {
    pub fn new(value: String) -> Option<Style> {
        let res = Style {
            value,
            color_code: "".to_string(),
        };

        if res
            .value
            .split(',')
            .map(|c| res.get_color_code(c, &Palette::default()))
            .any(|c| c.is_none())
        {
            return None;
        }
        Some(res)
    }

    fn get_color(&self, s: Option<&str>, palette: &Palette) -> Option<Color> {
        let color = match s {
            Some("black") => Some(palette.black),
            Some("red") => Some(palette.red),
            Some("green") => Some(palette.green),
            Some("yellow") => Some(palette.yellow),
            Some("blue") => Some(palette.blue),
            Some("magenta") => Some(palette.magenta),
            Some("cyan") => Some(palette.cyan),
            Some("white") => Some(palette.white),
            Some("orange") => Some(palette.orange),
            Some("gray") => Some(palette.gray),
            Some("purple") => Some(palette.purple),
            Some("gold") => Some(palette.gold),
            Some("silver") => Some(palette.silver),
            Some("pink") => Some(palette.pink),
            Some("brown") => Some(palette.brown),
            _ => None,
        };

        match color {
            Some(PaletteColor::Rgb((r, g, b))) => Some(Color::RGB(r, g, b)),
            Some(PaletteColor::EightBit(color)) => Some(Color::Fixed(color)),
            None => None,
        }
    }

    fn get_color_code(&self, s: &str, palette: &Palette) -> Option<String> {
        let mut iter = s.split(':').fuse();

        match iter.next() {
            Some("default") => Some(ansi_style::new().on(Color::Fixed(0)).suffix().to_string()),
            Some("bold") => Some(ansi_style::new().bold().prefix().to_string()),
            Some("fg") => match self.get_color(iter.next(), palette) {
                Some(c) => Some(ansi_style::new().fg(c).prefix().to_string()),
                None => None,
            },
            Some("bg") => match self.get_color(iter.next(), palette) {
                Some(c) => Some(ansi_style::new().on(c).prefix().to_string()),
                None => None,
            },
            _ => None,
        }
    }
}

impl Component for Style {
    fn get(&self) -> &str {
        &self.color_code
    }

    fn len(&self) -> usize {
        0
    }

    fn update(&mut self, event: &Event, _config: &Config) {
        if let Event::ModeUpdate(mode_info) = event {
            let palette = mode_info.style.colors;
            self.color_code = self
                .value
                .split(',')
                .map(|c| self.get_color_code(c, &palette).unwrap())
                .collect::<String>();
        }
    }
}

// Style ----------------------------------------------------------------------
#[derive(Default)]
pub struct Mode {
    value: String,
    len: usize,
}

impl Mode {
    pub fn new() -> Mode {
        Mode {
            ..Default::default()
        }
    }
}

impl Component for Mode {
    fn get(&self) -> &str {
        &self.value
    }

    fn len(&self) -> usize {
        self.len
    }

    fn update(&mut self, event: &Event, config: &Config) {
        if let Event::ModeUpdate(mode_info) = event {
            self.value = parse(&config.mode[&mode_info.mode])
                .iter_mut()
                .map(|x| {
                    x.update(event, config);
                    x.get()
                })
                .collect::<String>();
        }
    }
}
