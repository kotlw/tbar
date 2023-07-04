use crate::config::Config;
use crate::parser::{Color, Component, Style};
use zellij_tile::prelude::*;

#[derive(Default)]
pub struct Composer {
    config: Config,
    components: Vec<Component>,

    session_name: String,
    mode: InputMode,
    palette: Palette,
}

impl Composer {
    pub fn new(config: Config, components: Vec<Component>) -> Composer {
        Composer {
            config,
            components,
            ..Default::default()
        }
    }

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

    fn render_style(&self, style: &Style) -> String {
        let s = ansi_term::Style::new();
        match style {
            Style::Fg(c) => s.fg(self.get_color(c)).prefix().to_string(),
            Style::Bg(c) => s.on(self.get_color(c)).prefix().to_string(),
            Style::Bold => s.bold().prefix().to_string(),
            Style::Default => s.on(ansi_term::Color::Fixed(0)).suffix().to_string(),
        }
    }

    fn render(&self, component: &Component) -> String {
        match component {
            Component::Text(t) => t.to_string(),
            Component::Style(s) => self.render_style(s),
            Component::Session => self.session_name.clone(),
            Component::Mode => "{Mode}".to_string(),
        }
    }

    pub fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                if let Some(n) = mode_info.session_name {
                    if self.session_name != n {
                        should_render = true;
                        self.session_name = n;
                    }
                }
                if self.mode != mode_info.mode {
                    should_render = true;
                    self.mode = mode_info.mode;
                }
                if self.palette != mode_info.style.colors {
                    should_render = true;
                    self.palette = mode_info.style.colors
                }
            }
            _ => {}
        }
        should_render
    }

    pub fn compose(&self, cols: usize) -> String {
        let mut res = String::new();

        for c in self.components.iter() {
            res.push_str(&self.render(c))
        }

        res
    }
}
