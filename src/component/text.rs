use zellij_tile::prelude::*;

use crate::component::style::Style;
use crate::component::traits::Component;

pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: String, style: Style) -> Box<Text> {
        Box::new(Text {
            text: style.apply(text),
        })
    }
}

impl Component for Text {
    fn load(&mut self) {}

    fn update(&mut self, _event: Event) -> bool {
        false
    }

    fn render(&self) -> &String {
        &self.text
    }
}
