use colored::{Color, Colorize};

#[derive(Default)]
pub struct Style {
    padding: (usize, usize),
    bold: bool,
    color: Option<Color>,
    on_color: Option<Color>,
}

impl Style {
    pub fn new(
        padding: (usize, usize),
        bold: bool,
        color: Option<Color>,
        on_color: Option<Color>,
    ) -> Style {
        Style {
            padding,
            bold,
            color,
            on_color,
        }
    }

    pub fn apply(&self, string: String) -> String {
        let mut result: String;

        let left_padding = " ".repeat(self.padding.0);
        let right_padding = " ".repeat(self.padding.1);
        result = format!("{}{}{}", left_padding, string, right_padding);

        if self.bold {
            result = result.bold().to_string();
        }

        if self.color.is_some() {
            result = result.color(self.color.unwrap()).to_string();
        }

        if self.on_color.is_some() {
            result = result.on_color(self.on_color.unwrap()).to_string();
        }

        result
    }
}
