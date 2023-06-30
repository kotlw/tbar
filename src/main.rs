mod component;
mod config;
mod style;
mod tokenizer;

use zellij_tile::prelude::*;
use zellij_tile_utils::style;

use crate::config::Config;
use crate::tokenizer::{tokenize, Kind, Token};

#[derive(Default)]
struct State {
    config: Config,
    mode_info: ModeInfo,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        self.config = Config::default();
        set_selectable(false);
        subscribe(&[EventType::ModeUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    should_render = true;
                }
                self.mode_info = mode_info
            }
            _ => {}
        }
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let tokens = tokenize(&self.config.layout);
        let mut components: Vec<Box<dyn component::Component>> = Vec::new();

        let mut s = String::new();
        let palette = self.mode_info.style.colors;
        let mode_style = &self.config.mode_style;

        for t in tokens.iter() {
            components.push(match t.kind {
                Kind::Text => Box::new(component::Text::from_token(t)),
                Kind::Session => {
                    let tok = Token::new(
                        Kind::Text,
                        self.mode_info.session_name.as_ref().unwrap().to_string(),
                    );
                    Box::new(component::Text::from_token(&tok))
                }
                Kind::Mode => Box::new(component::Mode::from_cfg(mode_style, palette)),
                Kind::Style => Box::new(component::Style::from_token(t, palette)),
            });
        }

        for c in components.iter() {
            s.push_str(c.get());
        }

        // for component in components.iter() {
        //     let rendered = match component.kind {
        //         Kind::Text => Some(component.value.clone()),
        //         Kind::Session => self.mode_info.session_name.clone(),
        //         Kind::Mode => Some(mmode.clone()),
        //         Kind::Style => Some(style::apply(&component.value, self.mode_info.style.colors)),
        //     };
        //     if let Some(r) = rendered {
        //         s.push_str(&r);
        //     }
        // }

        print!("{}", s);
    }
}
