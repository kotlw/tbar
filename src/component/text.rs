use zellij_tile::prelude::*;

use crate::component::traits::Component;
use colored::Color;
use colored::Colorize;

pub struct Text {
    text: String,
}

impl Text {
    pub fn new(
        mut text: String,
        trim_spaces: bool,
        bold: bool,
        mut separator: String,
        color: Option<Color>,
        on_color: Option<Color>,
    ) -> Box<Text> {
        if !trim_spaces {
            text = format!(" {} ", text);
        }

        if bold {
            text = text.bold().to_string();
        }

        if color.is_some() {
            text = text.color(color.unwrap()).to_string();
        }

        if on_color.is_some() {
            text = text.on_color(on_color.unwrap()).to_string();
            separator = separator.color(on_color.unwrap()).to_string();
        }

        text = format!("{}{}", text, separator);

        Box::new(Text { text })
    }
}

impl Component for Text {
    fn load(&mut self) {}

    fn update(&mut self, _event: Event) -> bool {
        false
    }

    fn render(&self) -> String {
        self.text.clone()
    }
}
