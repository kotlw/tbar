use crate::component::traits::Component;
use zellij_tile::prelude::*;

pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: String) -> Text {
        Text { text }
    }
}

impl Component for Text {

    fn load(&mut self) {}

    fn update(&mut self, event: Event) -> bool {
        false
    }

    fn render(&self) -> String {
        self.text.clone()
    }
}

impl Default for Text {
    fn default() -> Self {
        Text {
            text: "def".to_string(),
        }
    }
}
