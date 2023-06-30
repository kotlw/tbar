#[derive(PartialEq, Debug)]
pub enum Kind {
    Session,
    Mode,
    Style,
    Text,
}

#[derive(Debug)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
}

impl Token {
    fn new(kind: Kind, value: String) -> Token {
        Token { kind, value }
    }
}

fn take_until(iter: impl Iterator<Item = char>, ch: char) -> String {
    iter.take_while(|&c| c != ch).collect::<String>()
}

pub fn tokenize(layout: &String) -> Vec<Token> {
    let mut res = Vec::new();
    let mut iter = layout.chars().peekable();

    while iter.peek().is_some() {
        // read text before first '#' char, push if not empty
        let value = take_until(iter.by_ref(), '#');
        if !value.is_empty() {
            res.push(Token::new(Kind::Text, value));
        }

        // then read text after '#' as different kind, repeat until end of the string
        let (kind, value) = match iter.next() {
            Some('S') => (Kind::Session, "".to_string()),
            Some('M') => (Kind::Mode, "".to_string()),
            Some('[') => (Kind::Style, take_until(iter.by_ref(), ']')),
            _ => (Kind::Text, "".to_string()),
        };
        if !(value.is_empty() && Kind::Text == kind) {
            res.push(Token::new(kind, value));
        }
    }

    res
}
