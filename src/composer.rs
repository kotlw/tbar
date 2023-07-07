use crate::config::Config;
use crate::parser::{Color, Component, Parser, Style};
use std::cmp;
use std::collections::HashMap;
use zellij_tile::prelude::*;

fn parse_modes(
    config: &Config,
) -> Result<(HashMap<InputMode, Vec<Component>>, usize), Vec<Component>> {
    let mut res = HashMap::new();
    let mut len = 0;

    // Parse Modes.
    for (k, v) in &config.mode {
        let components = Parser::new(&v)
            .style_and_text_only()
            .expect_parse("Error parsing mode: ");

        res.insert(*k, components);

        // Catching errors in modes config and calculating component len.
        if let Some(components) = res.get(&k) {
            for c in components {
                match c {
                    Component::ParseError {
                        hint,
                        layout,
                        hl_begin,
                        hl_end,
                    } => {
                        return Err(vec![Component::ParseError {
                            hint: hint.to_string(),
                            layout: layout.to_string(),
                            hl_begin: *hl_begin,
                            hl_end: *hl_end,
                        }]);
                    }
                    Component::Text(t) => len = cmp::max(len, t.chars().count()),
                    _ => (),
                }
            }
        }
    }

    Ok((res, len))
}

#[derive(Default)]
pub struct Composer {
    components: Vec<Component>,
    modes: HashMap<InputMode, Vec<Component>>,
    mode_len: usize,

    session_name: String,
    mode: InputMode,
    palette: Palette,
}

impl Composer {
    pub fn new(config: &Config, components: Vec<Component>) -> Composer {
        let mut components = components;
        let mut modes = HashMap::new();
        let mut mode_len = 0;

        match parse_modes(config) {
            Ok((m, l)) => {
                modes = m;
                mode_len = l
            }
            Err(c) => components = c,
        }

        Composer {
            components,
            modes,
            mode_len,
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

    /// Returns styled error message.
    fn render_error(
        &self,
        cols: usize,
        hint: &String,
        layout: &String,
        hl_begin: usize,
        hl_end: usize,
    ) -> String {
        let bg_color = self.render_style(&Style::Bg(Color::Red));
        let fg_color = self.render_style(&Style::Fg(Color::Black));
        let hl_color = self.render_style(&Style::Bg(Color::Yellow));

        let layout_len = layout.chars().count();
        let hint_len = hint.chars().count();
        let hl_len = hl_end.saturating_sub(hl_begin);
        let layout_bounds = cols.saturating_sub(hint_len);

        // Calculate layout window beginning and end
        let offset = layout_bounds.saturating_sub(hl_len + 6) / 2;
        let layout_begin = hl_begin.saturating_sub(offset);
        let layout_end = cmp::min(layout_len, hl_end.saturating_add(offset));

        // Setup layout wrapping strings
        let layout_left = if layout_begin > 0 { "..." } else { "^" };
        let layout_right = if layout_end < layout_len { "..." } else { "$" };

        // Calculate spacer len. It to fill bar with color till the end of the string.
        let visible_layout_len = layout_end.saturating_sub(layout_begin);
        let layout_wrapps_len = layout_left.chars().count() + layout_right.chars().count();
        let spacer_len = cols.saturating_sub(hint_len + visible_layout_len + layout_wrapps_len);
        let spacer = " ".repeat(spacer_len);

        // Squeeze highlighted text if needed.
        let squeeze_size = (hint_len + hl_len + 6).saturating_sub(cols);
        let hl_end_squeezed = cmp::max(hl_begin, hl_end.saturating_sub(squeeze_size));
        let mut highlight = hl_color.to_string() + &layout[hl_begin..hl_end_squeezed] + &bg_color;

        // Calculate offset (len of non displayable chars).
        let mut offset = bg_color.chars().count() + hl_color.chars().count();
        if squeeze_size == hl_begin {
            offset = 0;
            highlight = "".to_string();
        };

        let before_hl = layout_left.to_string() + &layout[layout_begin..hl_begin];
        let after_hl = layout[hl_end..layout_end].to_string() + &layout_right;
        let msg = hint.to_string() + &before_hl + &highlight + &after_hl + &spacer;

        bg_color + &fg_color + &msg.chars().take(cols + offset).collect::<String>()
    }

    fn render_mode(&self) -> String {
        let mut res = String::new();

        for c in &self.modes[&self.mode] {
            match c {
                Component::Text(t) => res.push_str(t),
                Component::Style(s) => res.push_str(&self.render_style(s)),
                _ => (),
            };
        }

        res
    }

    fn render(&self, component: &Component, cols: usize) -> String {
        match component {
            Component::Text(t) => t.to_string(),
            Component::Style(s) => self.render_style(s),
            Component::Session => self.session_name.clone(),
            Component::Mode => self.render_mode(),
            Component::ParseError {
                hint,
                layout,
                hl_begin,
                hl_end,
            } => self.render_error(cols, hint, layout, *hl_begin, *hl_end),
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
