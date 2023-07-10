mod composer;
mod config;
mod error;
mod mode;
mod parser;
mod style;
mod tab;

use zellij_tile::prelude::*;

use crate::composer::{Component, Composer};
use crate::config::Config;
use crate::parser::Parser;

#[derive(Default)]
struct State {
    composer: Composer,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        // it will be possible to parse zellij config in the future
        let cfg = Config::default();
        let components = Parser::new(&cfg.layout, "[SMT")
            .parse()
            .unwrap_or_else(|mut e| {
                e.add_context("Error parsing layout: ");
                vec![Component::from(e)]
            });
        self.composer = Composer::new(&cfg, components);
        set_selectable(false);
        subscribe(&[EventType::ModeUpdate, EventType::TabUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ModeUpdate(mode_info) => self.composer.update_mode(mode_info),
            Event::TabUpdate(tabs) => self.composer.update_tab(tabs),
            _ => {
                eprintln!("Got unrecognized event: {:?}", event);
                false
            }
        }
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        print!("{}", self.composer.compose(cols));
    }
}
