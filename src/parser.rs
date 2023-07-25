use core::iter::Enumerate;
use core::str::Chars;
use std::iter::Peekable;

/// Set of components to show in the bar.
#[derive(Debug, PartialEq)]
pub enum Component {
    Text(String),
    Style(Style),
    Spacer,
    Session,
    Mode,
    TabBar,
    SwapLayout,
    Index,
    Name,
    /// Layout string with highlighted part describing where parsing fails.
    LayoutHighlight {
        layout: String,
        hl_begin: usize,
        hl_end: usize,
    },
}

#[derive(Debug, PartialEq)]
pub enum Style {
    Bg(Color),
    Fg(Color),
    Bold,
    Default,
}

#[derive(Debug, PartialEq)]
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

/// Parsing error data.
#[derive(Debug)]
pub struct ParseError {
    pub context: String,
    pub layout: String,
    pub hl_begin: usize,
    pub hl_end: usize,
}

pub struct Parser<'a> {
    layout: &'a str,
    iter: Peekable<Enumerate<Chars<'a>>>,
    allowed_specials: Vec<Component>,
}

impl<'a> Parser<'a> {
    pub fn new(layout: &'a str, allowed_specials: Vec<Component>) -> Parser<'a> {
        Parser {
            layout,
            iter: layout.chars().enumerate().peekable(),
            allowed_specials,
        }
    }

    /// Parse color.
    fn parse_color(token: &str) -> Result<Style, &str> {
        let v: Vec<&str> = token.split(':').collect();

        let color = match v.get(1).cloned() {
            Some("black") => Color::Black,
            Some("red") => Color::Red,
            Some("green") => Color::Green,
            Some("yellow") => Color::Yellow,
            Some("blue") => Color::Blue,
            Some("magenta") => Color::Magenta,
            Some("cyan") => Color::Cyan,
            Some("white") => Color::White,
            Some("orange") => Color::Orange,
            Some("gray") => Color::Gray,
            Some("purple") => Color::Purple,
            Some("gold") => Color::Gold,
            Some("silver") => Color::Silver,
            Some("pink") => Color::Pink,
            Some("brown") => Color::Brown,
            _ => {
                return Err("Unknown color: ");
            }
        };

        match v.get(0).cloned() {
            Some("fg") => Ok(Style::Fg(color)),
            Some("bg") => Ok(Style::Bg(color)),
            _ => Err("Unknown color: "),
        }
    }

    /// Parse style inside #[].
    fn parse_style(token: &str) -> Result<Style, &str> {
        match token {
            "default" => Ok(Style::Default),
            "bold" => Ok(Style::Bold),
            _ if token.contains(':') => Ok(Self::parse_color(token)?),
            _ => Err("Unknown style: "),
        }
    }

    /// Parse style components.
    fn parse_style_group(&mut self) -> Result<Vec<Component>, ParseError> {
        let mut res = Vec::new();
        let hl_begin = self.iter.next().map(|(i, _)| i).unwrap(); // skips '['

        loop {
            let token = self.take_until_any(",]#");
            let style = Self::parse_style(&token);

            if let Ok(s) = style {
                res.push(Component::Style(s));
                match self.iter.peek() {
                    Some((_, ',')) => self.iter.next(),
                    Some((_, ']')) => return Ok(res),
                    _ => {
                        return Err(ParseError {
                            context: "Unclosed bracket: ".to_string(),
                            layout: self.layout.to_string(),
                            hl_begin,
                            hl_end: hl_begin + 1,
                        })
                    }
                };
            } else if let Err(e) = style {
                match self.iter.peek() {
                    Some((i, ',')) | Some((i, ']')) => {
                        return Err(ParseError {
                            context: e.to_string(),
                            layout: self.layout.to_string(),
                            hl_begin: i.saturating_sub(token.chars().count()),
                            hl_end: *i,
                        })
                    }
                    Some((_, '#')) | _ => {
                        return Err(ParseError {
                            context: "Unclosed bracket: ".to_string(),
                            layout: self.layout.to_string(),
                            hl_begin,
                            hl_end: hl_begin + 1,
                        })
                    }
                }
            }
        }
    }

    /// Parse non text components.
    fn parse_specials(&mut self) -> Result<Vec<Component>, ParseError> {
        macro_rules! is_allowed {
            ($($component:tt)+) => {
                self.allowed_specials.iter().any(|x| matches!(x, $($component)+))
            };
        }

        self.iter.next(); // skip '#' symbol

        let res = match self.iter.peek() {
            Some((_, 'S')) if is_allowed!(Component::Session) => Ok(vec![Component::Session]),
            Some((_, 'M')) if is_allowed!(Component::Mode) => Ok(vec![Component::Mode]),
            Some((_, 'T')) if is_allowed!(Component::TabBar) => Ok(vec![Component::TabBar]),
            Some((_, 'I')) if is_allowed!(Component::Index) => Ok(vec![Component::Index]),
            Some((_, 'N')) if is_allowed!(Component::Name) => Ok(vec![Component::Name]),
            Some((_, 'L')) if is_allowed!(Component::SwapLayout) => Ok(vec![Component::SwapLayout]),
            Some((_, '_')) if is_allowed!(Component::Spacer) => Ok(vec![Component::Spacer]),
            Some((_, '[')) if is_allowed!(Component::Style(..)) => Ok(self.parse_style_group()?),
            Some((hl_begin, _)) => Err(ParseError {
                context: "Unexpected token: ".to_string(),
                layout: self.layout.to_string(),
                hl_begin: *hl_begin,
                hl_end: *hl_begin + 1,
            }),
            None => Err(ParseError {
                context: "Unexpected token: ".to_string(),
                layout: self.layout.to_string(),
                hl_begin: self.layout.chars().count() - 1,
                hl_end: self.layout.chars().count(),
            }),
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
