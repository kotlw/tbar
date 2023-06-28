#[derive(Debug)]
enum TokenType {
    Session,
    Mode,
    Style,
    Command,
    Text,
}

#[derive(Debug)]
struct Token {
    _type: TokenType,
    value: String,
}

impl Token {
    fn new(_type: TokenType, value: Option<String>) -> Token {
        match _type {
            TokenType::Session => Token {
                _type,
                value: "".to_string(),
            },
            TokenType::Mode => Token {
                _type,
                value: "".to_string(),
            },
            TokenType::Style => Token {
                _type,
                value: value.unwrap(),
            },
            TokenType::Command => Token {
                _type,
                value: value.unwrap(),
            },
            TokenType::Text => Token {
                _type,
                value: value.unwrap(),
            },
        }
    }
}

fn tokenize(line: &String) -> Vec<Token> {
    let mut res = Vec::new();
    let mut iter = line.chars().peekable();

    while let Some(..) = iter.peek() {
        res.push(Token::new(
            TokenType::Text,
            Some(iter.by_ref().take_while(|&c| c != '#').collect::<String>()),
        ));
        match iter.next() {
            Some('S') => res.push(Token::new(TokenType::Session, None)),
            Some('M') => res.push(Token::new(TokenType::Mode, None)),
            Some('[') => res.push(Token::new(
                TokenType::Style,
                Some(iter.by_ref().take_while(|&c| c != ']').collect())
            )),
            Some('(') => res.push(Token::new(
                TokenType::Command,
                Some(iter.by_ref().take_while(|&c| c != ')').collect())
            )),
            _ => (),
        }
    }

    res
}
