use crate::composer::Component;
use crate::parser::{ParseError, Parser};
use crate::style::{Style, StyleRenderer};
use std::collections::HashMap;
use zellij_tile::prelude::*;

#[derive(Default)]
pub struct ModeRenderer {
    mode: InputMode,
    parsed_modes: HashMap<InputMode, Vec<Component>>,
    mode_len: usize,

    style_renderer: StyleRenderer,
}

impl ModeRenderer {
    pub fn new(modes_config: &HashMap<InputMode, String>) -> Result<ModeRenderer, ParseError> {
        let mut parsed_modes = HashMap::new();
        let mut mode_len = 0;

        for (k, v) in modes_config {
            let components = Parser::new(&v, "[").parse()?;
            let actual_len = components.iter().map(|c| match c {
                Component::Text(t) => t.chars().count(),
                _ => 0,
            }).sum();
            parsed_modes.insert(*k, components);
            mode_len = std::cmp::max(mode_len, actual_len)
        }

        Ok(ModeRenderer {
            parsed_modes,
            mode_len,
            ..Default::default()
        })
    }

    fn fit_mode_len(&self, t: &str) -> String {
        let total = self.mode_len - t.chars().count();
        let before = " ".repeat(total / 2);
        let after = " ".repeat(total - total / 2);
        before.to_string() + t + &after
    }

    pub fn update(&mut self, mode: InputMode, palette: Palette) -> bool {
        self.style_renderer.update(palette);
        if self.mode != mode {
            self.mode = mode;
            return true;
        }
        false
    }

    pub fn render(&self) -> String {
        let mut res = String::new();

        for c in &self.parsed_modes[&self.mode] {
            match c {
                Component::Text(t) => res.push_str(&self.fit_mode_len(t)),
                Component::Style(s) => res.push_str(&self.style_renderer.render(s)),
                _ => (),
            };
        }

        res.push_str(&self.style_renderer.render(&Style::Default));
        res
    }
}
