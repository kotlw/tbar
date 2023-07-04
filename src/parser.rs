use core::iter::Enumerate;
use core::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Component {
    Text(String),
    Style(Style),
    Session,
    Mode,
    Error(String, String, usize, usize),
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
}

impl<'a> Parser<'a> {
    pub fn new(layout: &'a str) -> Parser<'a> {
        let iter = layout.chars().enumerate().peekable();
        Parser { layout, iter }
    }

    /// Reads word from stream and select corresponding Color if exists, otherwise returns error.
    fn take_color(&mut self) -> Result<Color, (String, usize, usize)> {
        let mut color = String::with_capacity(8);
        let mut begin = self.layout.chars().count() - 1;
        let mut end = self.layout.chars().count();

        while let Some((i, c)) = self.iter.peek() {
            if begin > *i {
                begin = *i;
            }
            end = *i;
            if !c.is_alphanumeric() {
                break;
            }
            color.push(*c);
            self.iter.next();
        }

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
            _ => Err(("Unknown color".to_string(), begin, end)),
        }
    }

    /// Return style as Vec<Component> or error with highlight coordinates if pattern unrecognized.
    fn take_styles(&mut self) -> Result<Vec<Component>, (String, usize, usize)> {
        let mut res = Vec::new();
        let mut s = String::new();
        let mut begin = self.layout.chars().count() - 1;
        let mut end = self.layout.chars().count();

        while let Some((i, c)) = self.iter.peek() {
            if begin > *i {
                begin = *i;
            }
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
    fn take_specials(&mut self) -> Result<Vec<Component>, (String, usize, usize)> {
        let len = self.layout.chars().count();
        match self.iter.peek() {
            Some((_, 'S')) => {
                self.iter.next(); // skip 'S' symbol
                Ok(vec![Component::Session])
            }
            Some((_, 'M')) => {
                self.iter.next(); // skip 'M' symbol
                Ok(vec![Component::Mode])
            }
            Some((_, '[')) => {
                self.iter.next(); // skip '[' symbol
                Ok(self.take_styles()?)
            }
            Some((begin, _)) => Err(("Unexpected token".to_string(), *begin, *begin + 1)),
            // handle '#' if it's the last symbol
            None => Err(("Unexpected token".to_string(), len - 1, len)),
        }
    }

    /// Returns text untill special symbol '#' and moves iterator peek to it.
    fn take_text(&mut self) -> Result<Vec<Component>, (String, usize, usize)> {
        let mut res = String::new();

        while let Some((_, c)) = self.iter.peek() {
            if *c == '#' {
                break;
            }
            res.push(*c);
            self.iter.next();
        }
        Ok(vec![Component::Text(res)])
    }

    /// Returns Vec<Component> of layout if success, or Vec<Component> with error message. It needs
    /// because parser could be used in different scenarious so it needs different error prefix.
    pub fn parse(&mut self) -> Result<Vec<Component>, Component> {
        let mut res = Vec::new();

        while let Some((_, c)) = self.iter.peek() {
            let component = match c {
                '#' => {
                    self.iter.next();
                    self.take_specials()
                }
                _ => self.take_text(),
            };
            match component {
                Ok(c) => res.extend(c),
                Err((e, begin, end)) => {
                    return Err(Component::Error(self.layout.to_string(), e, begin, end))
                }
            };
        }
        Ok(res)
    }
}
