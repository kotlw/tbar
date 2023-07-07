use crate::config::Config;
use crate::error::ErrorRenderer;
use crate::mode::ModeRenderer;
use crate::style::{Style, StyleRenderer};
use zellij_tile::prelude::*;

#[derive(Debug)]
pub enum Component {
    Text(String),
    Style(Style),
    Session,
    Mode,
    ParseError {
        hint: String,
        layout: String,
        hl_begin: usize,
        hl_end: usize,
    },
}

#[derive(Default)]
pub struct Composer {
    components: Vec<Component>,

    session_name: String,
    style_renderer: StyleRenderer,
    mode_renderer: ModeRenderer,
    error_renderer: ErrorRenderer,
}

impl Composer {
    pub fn new(config: &Config, components: Vec<Component>) -> Composer {
        let mut components = components;
        let mut mode_renderer = ModeRenderer::default();

        match ModeRenderer::new(&config.mode) {
            Ok(m) => mode_renderer = m,
            Err(c) => components = c,
        }

        Composer {
            components,
            mode_renderer,
            ..Default::default()
        }
    }

    fn render(&self, component: &Component, cols: usize) -> String {
        match component {
            Component::Text(t) => t.to_string(),
            Component::Style(s) => self.style_renderer.render(s),
            Component::Session => self.session_name.clone(),
            Component::Mode => self.mode_renderer.render(),
            Component::ParseError {
                hint,
                layout,
                hl_begin,
                hl_end,
            } => self
                .error_renderer
                .render(cols, hint, layout, *hl_begin, *hl_end),
        }
    }

    fn update_session_name(&mut self, session_name: Option<String>) -> bool {
        if let Some(n) = session_name {
            if self.session_name != n {
                self.session_name = n;
                return true;
            }
        }
        false
    }

    pub fn update_mode(&mut self, mode_info: ModeInfo) -> bool {
        let mut should_render = false;
        let palette = mode_info.style.colors;
        should_render |= self.update_session_name(mode_info.session_name);
        should_render |= self.mode_renderer.update(mode_info.mode, palette);
        should_render |= self.style_renderer.update(palette);
        should_render |= self.error_renderer.update(palette);
        should_render
    }

    pub fn compose(&self, cols: usize) -> String {
        let mut res = String::new();

        for c in self.components.iter() {
            res.push_str(&self.render(c, cols))
        }

        res
    }
}
