mod composer;
mod config;
mod parser;

use zellij_tile::prelude::*;

use crate::config::Config;
use crate::parser::Parser;
use crate::composer::Composer;

#[derive(Default)]
struct State {
    composer: Composer,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        // it will be possible to parse zellij config in the future
        let c = Config::default();
        let p = Parser::new(&c.layout).parse();
        self.composer = Composer::new(c, p);
        set_selectable(false);
        subscribe(&[EventType::ModeUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        self.composer.update(event)
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        let s = self.composer.compose(cols);
        print!("{}", s);
    }
}
