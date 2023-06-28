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

        if let Some(color) = self.color {
            result = result.color(color).to_string();
        }

        if let Some(color) = self.on_color {
            result = result.on_color(color).to_string();
        }

        result
    }
}