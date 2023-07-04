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

    fn render_error(
        &self,
        cols: usize,
        layout: &String,
        hint: &String,
        begin: usize,
        end: usize,
    ) -> String {
        let bg = self.render_style(&Style::Bg(Color::Red));
        let fg = self.render_style(&Style::Fg(Color::Black));
        let hl = self.render_style(&Style::Bg(Color::Yellow));

        if cols <= hint.chars().count() + 8 {
            return format!(
                "{}{}{}",
                bg,
                fg,
                (hint.to_string() + ": ......")
                    .chars()
                    .take(cols)
                    .collect::<String>()
            );
        }

        let layout_len = layout.chars().count();
        let hint_len = hint.chars().count() + 2; // + 2 symbols ': '
        let bounds = cols.saturating_sub(hint_len);

        let window = bounds.saturating_sub(end.saturating_sub(begin) + 6) / 2;
        let b = begin.saturating_sub(window);
        let e = std::cmp::min(layout_len, end.saturating_add(window));

        let layout_msg = format!(
            "{}{}{}{}{}{}{}",
            if b > 0 { "..." } else { "^" }.to_string(),
            layout[b..begin].to_string(),
            hl,
            layout[begin..end].to_string(),
            bg,
            layout[end..e].to_string(),
            if e < layout_len { "..." } else { "$" }.to_string()
        );

        let mut cols_left = cols.saturating_sub(hint_len + 2 + e.saturating_sub(b));
        if b > 0 {
            cols_left = cols_left.saturating_sub(2);
        }
        if e < layout_len {
            cols_left = cols_left.saturating_sub(2);
        }
        let spacer = if cols_left > 0 {
            " ".to_string().repeat(cols_left)
        } else {
            "".to_string()
        };

        format!("{}{}{}: {}{}", bg, fg, hint, layout_msg, spacer)
    }

    fn render(&self, component: &Component, cols: usize) -> String {
        match component {
            Component::Text(t) => t.to_string(),
            Component::Style(s) => self.render_style(s),
            Component::Session => self.session_name.clone(),
            Component::Mode => "{Mode}".to_string(),
            Component::Error(l, h, b, e) => self.render_error(cols, l, h, *b, *e),
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
            res.push_str(&self.render(c, cols))
        }

        res
    }
}
