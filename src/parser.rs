use crate::component::{Component, Session, Style, Text, Mode};

fn error_highlight(value: String) -> Vec<Box<dyn Component>> {
    vec![
        Box::new(Style::new("bg:red,fg:black".to_string()).unwrap()),
        Box::new(Text::new(value)),
        Box::new(Style::new("default".to_string()).unwrap()),
    ]
}

fn take_until(iter: impl Iterator<Item = char>, ch: char) -> String {
    iter.take_while(|&c| c != ch).collect::<String>()
}

pub fn parse(layout: &String) -> Vec<Box<dyn Component>> {
    let mut res: Vec<Box<dyn Component>> = Vec::new();
    let mut iter = layout.chars().peekable();

    while iter.peek().is_some() {
        // read text before first '#' char, push if not empty
        let value = take_until(iter.by_ref(), '#');
        if !value.is_empty() {
            res.push(Box::new(Text::new(value)));
        }

        // then read text after '#' as different kind, repeat until end of the string
        if let Some(c) = iter.next() {
            match c {
                'S' => res.push(Box::new(Session::default())),
                'M' => res.push(Box::new(Mode::default())),
                '[' => {
                    let value = take_until(iter.by_ref(), ']');
                    let style = Style::new(value.clone());
                    match style {
                        Some(s) => res.push(Box::new(s)),
                        None => res.extend(error_highlight(format!("#[{}]", value))),
                    }
                }
                _ => res.extend(error_highlight(format!("#{}", c))),
            };
        }
    }

    res
}
