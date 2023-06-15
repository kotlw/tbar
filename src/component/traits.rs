use zellij_tile::prelude::*;

pub trait Component {
    fn load(&mut self) {}
    fn update(&mut self, event: Event) -> bool {
        false
    } // return true if it should render
    fn render(&self) -> String { "d".to_string() }
}
