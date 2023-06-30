mod config;
mod tokenizer;
mod style;
mod component;

use zellij_tile::prelude::*;
use zellij_tile_utils::style;
use ansi_term::ANSIStrings;


use crate::config::Config;
use crate::tokenizer::{tokenize, Kind};

#[derive(Default)]
struct State {
    options: Config,
    mode_info: ModeInfo,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        self.options = Config::default();
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
        let components = tokenize(&self.options.layout);

        let mut s = String::new();
        let mmode = "{mode}".to_string();

        for component in components.iter() {
            let rendered = match component.kind {
                Kind::Text => Some(component.value.clone()),
                Kind::Session => self.mode_info.session_name.clone(),
                Kind::Mode => Some(mmode.clone()),
                Kind::Style => Some(style::apply(&component.value, self.mode_info.style.colors)),
            };
            if let Some(r) = rendered {
                s.push_str(&r);
            }
        }

        print!("{}", s);
    }
}
