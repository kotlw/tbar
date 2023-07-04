mod composer;
mod config;
mod parser;

use zellij_tile::prelude::*;

use crate::config::Config;
use crate::parser::{Parser, Component, Style, Color};
use crate::composer::Composer;

#[derive(Default)]
struct State {
    composer: Composer,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        // it will be possible to parse zellij config in the future
        let cfg = Config::default();
        let components = Parser::new(&cfg.layout).expect_parse("Error while parsing layout: ");
        self.composer = Composer::new(cfg, components);
        set_selectable(false);
        subscribe(&[EventType::ModeUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        self.composer.update(event)
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        print!("{}", self.composer.compose(cols));
    }
}
