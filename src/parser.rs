use crate::composer::Component;
use crate::style::{Color, Style};
use core::iter::Enumerate;
use core::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub struct ParseError {
    pub context: String,
    pub layout: String,
    pub hl_begin: usize,
    pub hl_end: usize,
}

impl ParseError {
    fn new(context: &str, layout: &str, hl_begin: usize, hl_end: usize) -> ParseError {
        ParseError {
            context: context.to_string(),
            layout: layout.to_string(),
            hl_begin,
            hl_end,
        }
    }

    pub fn add_context(&mut self, context: &str) {
        self.context.insert_str(0, context);
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
    fn parse_styles(&mut self) -> Result<Vec<Component>, ParseError> {
        let mut res = Vec::new();
        let hl_begin = self.iter.next().map(|(i, _)| i).unwrap_or(1); // skips '['

        loop {
            let token = self.take_until_any(",]#");
            let style = parse_style(&token);

            if let Ok(s) = style {
                res.push(Component::Style(s));
                match self.iter.peek() {
                    Some((_, ',')) => self.iter.next(),
                    Some((_, ']')) => return Ok(res),
                    _ => {
                        return Err(ParseError::new(
                            "Unclosed bracket: ",
                            self.layout,
                            hl_begin,
                            hl_begin + 1,
                        ))
                    }
                };
            } else if let Err(e) = style {
                match self.iter.peek() {
                    Some((i, ',')) | Some((i, ']')) => {
                        return Err(ParseError::new(
                            e,
                            self.layout,
                            i.saturating_sub(token.chars().count()),
                            *i,
                        ))
                    }
                    Some((_, '#')) | _ => {
                        return Err(ParseError::new(
                            "Unclosed bracket: ",
                            self.layout,
                            hl_begin,
                            hl_begin + 1,
                        ))
                    }
                }
            }
        }
    }

    /// Parse non text components.
    fn parse_specials(&mut self) -> Result<Vec<Component>, ParseError> {
        self.iter.next(); // skip '#' symbol

        let res = match self.iter.peek() {
            Some((_, 'S')) if self.allowed_specials.contains('S') => Ok(vec![Component::Session]),
            Some((_, 'M')) if self.allowed_specials.contains('M') => Ok(vec![Component::Mode]),
            Some((_, 'T')) if self.allowed_specials.contains('T') => Ok(vec![Component::Tab]),
            Some((_, 'I')) if self.allowed_specials.contains('I') => Ok(vec![Component::Index]),
            Some((_, 'N')) if self.allowed_specials.contains('N') => Ok(vec![Component::Name]),
            Some((_, '[')) if self.allowed_specials.contains('[') => Ok(self.parse_styles()?),
            Some((hl_begin, _)) => Err(ParseError::new(
                "Unexpected token: ",
                self.layout,
                *hl_begin,
                *hl_begin + 1,
            )),
            None => Err(ParseError::new(
                "Unexpected token: ",
                self.layout,
                self.len - 1,
                self.len,
            )),
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
                _ => return res,
            }
            self.iter.next();
        }
    }

    /// Parsing entrypoint, decides to parse text or special tokens.
    pub fn parse(&mut self) -> Result<Vec<Component>, ParseError> {
        let mut res = Vec::new();
        loop {
            match self.iter.peek() {
                Some((_, '#')) => res.extend(self.parse_specials()?),
                Some((_, _)) => res.push(Component::Text(self.take_until_any("#"))),
                None => return Ok(res),
            }
        }
    }
}
