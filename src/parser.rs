#[derive(Debug)]
pub enum Kind {
    Session,
    Mode,
    Style,
    Cmd,
    Text,
}

#[derive(Debug)]
pub struct Component {
    pub kind: Kind,
    pub value: String,
}

impl Component {
    fn new(kind: Kind, value: String) -> Component {
        Component { kind, value }
    }
}

fn take_until(iter: impl Iterator<Item = char>, ch: char) -> String {
    iter.take_while(|&c| c != ch).collect::<String>()
}

pub fn parse(layout: &String) -> Vec<Component> {
    let mut res = Vec::new();
    let mut iter = layout.chars().peekable();

    while iter.peek().is_some() {
        // read text before first '#' char
        res.push(Component::new(Kind::Text, take_until(iter.by_ref(), '#')));

        // then read text after '#' as different kind repeat until end of the string
        let (kind, value) = match iter.next() {
            Some('S') => (Kind::Session, "".to_string()),
            Some('M') => (Kind::Mode, "".to_string()),
            Some('[') => (Kind::Style, take_until(iter.by_ref(), ']')),
            Some('(') => (Kind::Cmd, take_until(iter.by_ref(), ')')),
            _ => (Kind::Text, "".to_string()),
        };
        res.push(Component::new(kind, value));
    }

    res
}
