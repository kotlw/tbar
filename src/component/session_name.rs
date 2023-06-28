use zellij_tile::prelude::*;

use unicode_width::UnicodeWidthStr;
use crate::component::style::Style;
use crate::component::traits::Component;

pub struct SessionName {
    session_name: String,
}

impl SessionName {
    pub fn new(session_name: String, style: Style) -> Box<SessionName> {
        Box::new(SessionName {
            session_name: style.apply(session_name),
        })
    }
}


impl Component for SessionName {
    fn load(&mut self) {}

    fn update(&mut self, _event: Event) -> bool {
        false
    }

    fn render(&self) -> &String {
        &self.session_name
    }

    fn width(&self) -> usize {
        self.session_name.width()
    }
}

