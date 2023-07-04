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
        let components = match Parser::new(&cfg.layout).parse() {
            Ok(c) => c,
            Err(c) => {
                if let Component::Error(l, h, b, e) = c {
                    let hint = "Error while parsing layout: ".to_string() + &h;
                    vec![Component::Error(l, hint, b, e)]
                } else {
                    // it's impossible to get here
                    vec![]
                }
            }, 
        };
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
