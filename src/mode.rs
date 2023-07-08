use crate::composer::Component;
use crate::parser::Parser;
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
    pub fn new(modes_config: &HashMap<InputMode, String>) -> Result<ModeRenderer, Vec<Component>> {
        let mut parsed_modes = HashMap::new();
        let mut mode_len = 0;

        // Parse Modes.
        for (k, v) in modes_config {
            let mode_components = Parser::new(&v, "[").expect_parse("Error parsing mode: ");
            parsed_modes.insert(*k, mode_components);

            // Catching errors in modes config and calculating component len.
            if let Some(components) = parsed_modes.get(&k) {
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
                        Component::Text(t) => mode_len = std::cmp::max(mode_len, t.chars().count()),
                        _ => (),
                    }
                }
            }
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
