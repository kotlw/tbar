use crate::composer::Component;
use crate::style::{Color, Style};
use core::iter::Enumerate;
use core::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
struct Error {
    hint: String,
    hl_begin: usize,
    hl_end: usize,
}

impl Error {
    fn new(hint: &str, hl_begin: usize, hl_end: usize) -> Error {
        Error {
            hint: hint.to_string(),
            hl_begin,
            hl_end,
        }
    }
}

/// Parse color.
fn parse_color(token: &str) -> Result<Style, &str> {
    let v: Vec<&str> = token.split(':').collect();

    let color = match v.get(1) {
        Some(&"black") => Color::Black,
        Some(&"red") => Color::Red,
        Some(&"green") => Color::Green,
        Some(&"yellow") => Color::Yellow,
        Some(&"blue") => Color::Blue,
        Some(&"magenta") => Color::Magenta,
        Some(&"cyan") => Color::Cyan,
        Some(&"white") => Color::White,
        Some(&"orange") => Color::Orange,
        Some(&"gray") => Color::Gray,
        Some(&"purple") => Color::Purple,
        Some(&"gold") => Color::Gold,
        Some(&"silver") => Color::Silver,
        Some(&"pink") => Color::Pink,
        Some(&"brown") => Color::Brown,
        _ => {
            return Err("Unknown color: ");
        }
    };

    match v.get(0) {
        Some(&"fg") => Ok(Style::Fg(color)),
        Some(&"bg") => Ok(Style::Bg(color)),
        _ => Err("Unknown color: "),
    }
}

/// Parse style inside #[].
fn parse_style(token: &str) -> Result<Style, &str> {
    match token {
        "default" => Ok(Style::Default),
        "bold" => Ok(Style::Bold),
        _ if token.contains(':') => Ok(parse_color(token)?),
        _ => Err("Unknown style: "),
    }
}

pub struct Parser<'a> {
    layout: &'a str,
    len: usize,
    iter: Peekable<Enumerate<Chars<'a>>>,
    allowed_specials: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(layout: &'a str, allowed_specials: &'a str) -> Parser<'a> {
        let iter = layout.chars().enumerate().peekable();
        let len = layout.chars().count();
        Parser {
            layout,
            len,
            iter,
            allowed_specials,
        }
    }

    /// Parse style components.
    fn parse_styles(&mut self) -> Result<Vec<Component>, Error> {
        let mut res = Vec::new();
        let hl_begin = self.iter.next().map(|(i, _)| i).unwrap_or(1); // skips '['

        loop {
            let token = self.take_until_any(",]#");

            match parse_style(&token) {
                Ok(style) => {
                    res.push(Component::Style(style));
                    match self.iter.peek() {
                        // style parsed and waiting for next one
                        Some((_, ',')) => self.iter.next(),
                        // style parsed and bracket is closed
                        Some((_, ']')) => return Ok(res),
                        // style parsed but bracket is not closed
                        _ => return Err(Error::new("Unclosed bracket: ", hl_begin, hl_begin + 12)),
                    };
                }
                Err(e) => {
                    match self.iter.peek() {
                        // style not parsed and waiting for next one or bracket is closed
                        Some((i, ',')) | Some((i, ']')) => {
                            return Err(Error::new(e, i.saturating_sub(token.chars().count()), *i))
                        }
                        // style not parsed because of unexpected '#' or end of the line
                        Some((_, '#')) | _ => {
                            return Err(Error::new("Unclosed bracket: ", hl_begin, hl_begin + 1))
                        }
                    }
                }
            }
        }
    }

    /// Parse non text components.
    fn parse_specials(&mut self) -> Result<Vec<Component>, Error> {
        self.iter.next(); // skip '#' symbol

        let res = match self.iter.peek() {
            Some((_, 'S')) if self.allowed_specials.contains('S') => Ok(vec![Component::Session]),
            Some((_, 'M')) if self.allowed_specials.contains('M') => Ok(vec![Component::Mode]),
            Some((_, 'T')) if self.allowed_specials.contains('T') => Ok(vec![Component::Tab]),
            Some((_, 'I')) if self.allowed_specials.contains('I') => Ok(vec![Component::Index]),
            Some((_, 'N')) if self.allowed_specials.contains('N') => Ok(vec![Component::Name]),
            Some((_, '[')) if self.allowed_specials.contains('[') => Ok(self.parse_styles()?),
            Some((hl_begin, _)) => Err(Error::new("Unexpected token: ", *hl_begin, *hl_begin + 1)),
            None => Err(Error::new("Unexpected token: ", self.len - 1, self.len)),
        };

        self.iter.next(); // it should be 'S' | 'M' | ']' ...
        res
    }

    /// Returns str until any of given tokens.
    fn take_until_any(&mut self, tokens: &str) -> String {
        let mut res = String::new();
        loop {
            match self.iter.peek() {
                Some((_, c)) if !tokens.contains(*c) => res.push(*c),
                _ => break res,
            }
            self.iter.next();
        }
    }

    /// Parsing entrypoint, decides to parse text or special tokens.
    fn parse(&mut self) -> Result<Vec<Component>, Error> {
        let mut res = Vec::new();

        while let Some((_, c)) = self.iter.peek() {
            match c {
                '#' => res.extend(self.parse_specials()?),
                _ => res.push(Component::Text(self.take_until_any("#"))),
            };
        }

        Ok(res)
    }

    /// Returns components to render. It will return vec![Component::Error] with details if
    /// it's failed to parse.
    pub fn expect_parse(&mut self, msg: &str) -> Vec<Component> {
        self.parse().unwrap_or_else(|e| {
            vec![Component::ParseError {
                layout: self.layout.to_string(),
                hint: msg.to_string() + &e.hint,
                hl_begin: e.hl_begin,
                hl_end: e.hl_end,
            }]
        })
    }
}
