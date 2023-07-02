mod component;
mod config;
mod parser;

use zellij_tile::prelude::*;

use crate::component::Component;
use crate::config::Config;
use crate::parser::parse;

#[derive(Default)]
struct State {
    config: Config,
    mode_info: ModeInfo,
    components: Vec<Box<dyn Component>>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        self.config = Config::default();
        self.components = parse(&self.config.layout);
        set_selectable(false);
        subscribe(&[EventType::ModeUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        let cfg = &self.config;
        self.components.iter_mut().for_each(|x| x.update(&event, cfg));
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
        let s = self.components
            .iter()
            .map(|p| p.get())
            .collect::<String>();

        print!("{}", s);
    }
}
