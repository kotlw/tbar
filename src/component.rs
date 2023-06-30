use crate::style;
use crate::tokenizer;
use std::collections::HashMap;
use zellij_tile::prelude::*;

pub trait Component {
    fn get(&self) -> &String;
    fn len(&self) -> usize;
}

pub struct Text {
    value: String,
}

impl Text {
    pub fn from_token(token: &tokenizer::Token) -> Text {
        Text { value: token.value.clone() }
    }
}

impl Component for Text {
    fn get(&self) -> &String {
        &self.value
    }

    fn len(&self) -> usize {
        self.value.chars().count()
    }
}

pub struct Style {
    value: String,
}

impl Style {
    pub fn from_token(token: &tokenizer::Token, palette: Palette) -> Style {
        Style {
            value: style::apply(&token.value, palette),
        }
    }
}

impl Component for Style {
    fn get(&self) -> &String {
        &self.value
    }

    fn len(&self) -> usize {
        0
    }
}

#[derive(Default)]
pub struct Mode {
    mode: InputMode,
    len: usize,
    value: HashMap<InputMode, String>,
}

impl Component for Mode {
    fn get(&self) -> &String {
        &self.value.get(&self.mode).unwrap()
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl Mode {
    pub fn from_cfg(mode_style: &HashMap<InputMode, String>, palette: Palette) -> Mode {
        let mut value = HashMap::new();
        let mut max_len = 0;

        for (key, val) in mode_style.iter() {
            let mut res = String::new();
            let mut curr_len = 0;

            for p in tokenizer::tokenize(&val.to_string()).iter() {
                match p.kind {
                    tokenizer::Kind::Style => res.push_str(&style::apply(&p.value, palette)),
                    tokenizer::Kind::Text => {
                        curr_len += p.value.chars().count();
                        res.push_str(&p.value)
                    }
                    _ => (),
                }
            }

            max_len = std::cmp::max(max_len, curr_len);
 
            // drop style
            res.push_str(
                &ansi_term::Style::new()
                    .on(ansi_term::Color::Fixed(0))
                    .suffix()
                    .to_string(),
            );

            value.insert(*key, res);
        }

        Mode {
            mode: InputMode::Normal,
            len: max_len,
            value,
        }
    }
    pub fn set_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }
}
