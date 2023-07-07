mod composer;
mod config;
mod mode;
mod parser;
mod style;
mod error;

use zellij_tile::prelude::*;

use crate::composer::Composer;
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
        let components = Parser::new(&cfg.layout).expect_parse("Error parsing layout: ");
        self.composer = Composer::new(&cfg, components);
        set_selectable(false);
        subscribe(&[EventType::ModeUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ModeUpdate(mode_info) => self.composer.update_mode(mode_info),
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
