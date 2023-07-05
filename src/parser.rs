use core::iter::Enumerate;
use core::str::Chars;
use std::iter::Peekable;

const UNEXPECTED_TOKEN: String = "Unexpected token".to_string();
const UNKNOWN_COLOR: String = "Unknown clolr".to_string();

#[derive(Debug)]
struct ParseError<'a> {
    hint: String,
    layout: &'a str,
    hl_begin: usize,
    hl_end: usize,
}

impl<'a> ParseError<'a> {
    fn prepend_hint(&mut self, msg: &str) -> Self {
        self.hint.insert_str(0, msg);
        *self
    }

    fn as_component(&self) -> Component {
        Component::ParseError {
            layout: self.layout,
            hint: self.hint,
            hl_begin: self.hl_begin,
            hl_end: self.hl_end,
        }
    }
}

#[derive(Debug)]
pub enum Component<'a> {
    Text(String),
    Style(Style),
    Session,
    Mode,
    ParseError {
        hint: String,
        layout: &'a str,
        hl_begin: usize,
        hl_end: usize,
    },
}

#[derive(Debug)]
pub enum Style {
    Bg(Color),
    Fg(Color),
    Bold,
    Default,
}

#[derive(Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Orange,
    Gray,
    Purple,
    Gold,
    Silver,
    Pink,
    Brown,
}

#[derive(Debug)]
pub struct Parser<'a> {
    layout: &'a str,
    iter: Peekable<Enumerate<Chars<'a>>>,
    style_and_text_only: bool,
}

impl<'a> Parser<'a> {
    pub fn new(layout: &'a str) -> Parser<'a> {
        let iter = layout.chars().enumerate().peekable();
        Parser {
            layout,
            iter,
            style_and_text_only: false,
        }
    }

    fn err(&self, hint: String, hl_begin: usize, hl_end: usize) -> &mut ParseError {
        &mut ParseError {
            layout: &self.layout,
            hint,
            hl_begin,
            hl_end,
        }
    }

    /// Reads word from stream and select corresponding Color if exists, otherwise returns error.
    fn take_color(&mut self) -> Result<Color, &mut ParseError> {
        let mut color = String::with_capacity(8);
        let mut hl_end = self.layout.chars().count();

        while let Some((i, c)) = self.iter.peek() {
            if !c.is_alphanumeric() {
                break;
            }
            hl_end = *i;
            color.push(*c);
            self.iter.next();
        }

        while let Some((i, c)) = self.iter.peek() {
            if c.is_alphanumeric() {
                hl_end = *i;
                color.push(*c);
                self.iter.next();
            } else {
                break;
            }
        }

        while self.iter.peek().is_some_and(|(_, c)| c.is_alphanumeric()) {
            if let Some((i, c)) = self.iter.peek() {
                hl_end = *i;
                color.push(*c);
                self.iter.next();
            }
        }

        let hl_begin = hl_end.saturating_sub(color.chars().count());
        match color.as_str() {
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            "orange" => Ok(Color::Orange),
            "gray" => Ok(Color::Gray),
            "purple" => Ok(Color::Purple),
            "gold" => Ok(Color::Gold),
            "silver" => Ok(Color::Silver),
            "pink" => Ok(Color::Pink),
            "brown" => Ok(Color::Brown),
            _ => Err(self.err(UNKNOWN_COLOR, hl_begin, hl_end)),
        }
    }

    fn take_color_style(&self, prefix: String) -> Result<Style, &mut ParseError> {
        self.iter.next(); // skip ':'
        match prefix.as_str() {
            "bg" => Ok(Style::Bg(self.take_color()?)),
            "fg" => Ok(Style::Fg(self.take_color()?)),
            _ => Err(self.err(UNEXPECTED_TOKEN, 1, 1)),
        }
    }

    /// Return style as Vec<Component> or error with highlight coordinates if pattern unrecognized.
    fn take_styles(&mut self) -> Result<Vec<Component>, &mut ParseError> {
        let mut res = Vec::new();
        let mut prefix = String::new();
        let mut hl_begin = self.layout.chars().count() - 1;

        if let Some((i, _)) = self.iter.next() {
            // skip '['
            if hl_begin > i {
                hl_begin = i
            }
        }

        while let Some((i, c)) = self.iter.peek() {
            match *c {
                ':' => res.push(self.take_color_style?(prefix)),
                _ => {
                    prefix.push(*c);
                    self.iter.next();
                }
            }
        }

        while let Some((i, c)) = self.iter.peek() {
            end = *i;
            if *c == ':' {
                self.iter.next();
                match s.as_str() {
                    "bg" => res.push(Component::Style(Style::Bg(self.take_color()?))),
                    "fg" => res.push(Component::Style(Style::Fg(self.take_color()?))),
                    _ => return Err(("Unexpected token".to_string(), begin, end)),
                };
                s.clear();
            } else if *c == ',' || *c == ']' {
                let b = *i - s.chars().count();
                match s.as_str() {
                    "default" => res.push(Component::Style(Style::Default)),
                    "bold" => res.push(Component::Style(Style::Bold)),
                    "" => {
                        if *c == ',' {
                            self.iter.next();
                        } else {
                            break;
                        }
                    }
                    _ => return Err(("Unexpected token".to_string(), b, end)),
                };
                s.clear();
            } else if *c == '#' {
                break;
            } else {
                s.push(*c);
                self.iter.next();
            }
        }

        if self.iter.peek().is_some_and(|(_, c)| *c == ']') {
            self.iter.next();
            if res.is_empty() {
                return Err(("Empty style".to_string(), begin - 2, end + 1));
            }
            return Ok(res);
        }
        Err(("Bracket never closed".to_string(), begin, end))
    }

    /// Returns component described after '#' symbol or error with higllight coordinates if unrecognized.
    fn take_specials(&mut self) -> Result<Vec<Component>, &mut ParseError> {
        self.iter.next(); // skip '#' symbol

        let len = self.layout.chars().count();
        let res = match self.iter.peek() {
            Some((_, 'S')) if !self.style_and_text_only => Ok(vec![Component::Session]),
            Some((_, 'M')) if !self.style_and_text_only => Ok(vec![Component::Mode]),
            Some((_, '[')) => Ok(self.take_styles()?),
            Some((hl_begin, _)) => Err(self.err(UNEXPECTED_TOKEN, *hl_begin, *hl_begin + 1)),
            None => Err(self.err(UNEXPECTED_TOKEN, len - 1, len)),
        };

        self.iter.next(); // it should be 'S' | 'M' | ']' ...
        res
    }

    /// Returns text untill special symbol '#' and moves iterator peek to it. Wrap into Vec because
    /// parsing style could return several components.
    fn take_text(&mut self, start: &usize) -> Vec<Component> {
        let mut res = String::new();
        loop {
            match self.iter.peek() {
                Some((_, c)) if *c != '#' => res.push(*c),
                _ => break vec![Component::Text(res)],
            }
            self.iter.next();
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Component>, &mut ParseError> {
        let mut res = Vec::new();

        while let Some((i, c)) = self.iter.peek() {
            res.extend(match c {
                '#' => self.take_specials()?,
                _ => self.take_text(i),
            });
        }

        Ok(res)
    }

    /// Returns components to render. It will return vec![Component::Error] with details if it's failed
    /// to parse.
    pub fn expect_parse(&mut self, msg: &str) -> Vec<Component> {
        match self.parse() {
            Ok(c) => c,
            Err(e) => vec![e.prepend_hint(msg).as_component()],
        }
    }

    /// Specify to parse only text and style, used for parsing mode.
    pub fn style_and_text_only(&mut self) -> &mut Self {
        self.style_and_text_only = true;
        self
    }
}
