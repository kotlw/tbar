mod component;
mod config;
mod options;

use zellij_tile::prelude::*;

use colored::Color;

use crate::component::text::Text;
use crate::component::traits::Component;
use crate::options::Options;
use crate::component::style::Style;

#[derive(Default)]
struct FooBar {
    options: Options,
    components: Vec<Box<dyn Component>>,
}

register_plugin!(FooBar);

impl ZellijPlugin for FooBar {
    fn load(&mut self) {
        set_selectable(false);
        self.components = vec![Text::new(
            "Hello world".to_string(),
            Style::new((1, 1), false, Some(Color::BrightGreen), None)
        )]
    }

    fn update(&mut self, event: Event) -> bool {
        false
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let mut s = String::new();
        for cmp in self.components.iter() {
            s = s + &cmp.render();
        }
        print!("{}", s);
    }
}
