use zellij_tile::prelude::*;

pub trait Component {
    fn load(&mut self);
    fn update(&mut self, event: Event) -> bool;
    fn render(&self) -> &String;
}
