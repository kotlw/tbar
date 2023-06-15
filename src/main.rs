mod config;
mod component;

use zellij_tile::prelude::*;

use crate::component::text::Text;
use crate::component::traits::Component;

#[derive(Default)]
struct FooBar {
    components: Vec<Box<dyn Component>>
}

register_plugin!(FooBar);

impl ZellijPlugin for FooBar {
    fn load(&mut self) {
        set_selectable(false);
        self.components.push(Box::new(Text::new("check".to_string())));
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
