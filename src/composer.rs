use crate::config::Config;
use crate::error::ErrorRenderer;
use crate::mode::ModeRenderer;
use crate::style::{Style, StyleRenderer};
use crate::tab::TabRenderer;
use crate::parser::ParseError;
use crate::State;
use zellij_tile::prelude::*;

#[derive(Debug)]
pub enum Component {
    Text(String),
    Style(Style),
    Session,
    Mode,
    Tab,
    Index,
    Name,
    ParseError {
        context: String,
        layout: String,
        hl_begin: usize,
        hl_end: usize,
    },
}

impl From<ParseError> for Component {
    fn from(value: ParseError) -> Self {
        Component::ParseError {
            context: value.context,
            layout: value.layout,
            hl_begin: value.hl_begin,
            hl_end: value.hl_end
        }
    }
} 

#[derive(Default)]
pub struct Composer {
    components: Vec<Component>,

    session_name: String,
    style_renderer: StyleRenderer,
    mode_renderer: ModeRenderer,
    tab_renderer: TabRenderer,
    error_renderer: ErrorRenderer,
}

impl Composer {
    pub fn new(config: &Config, components: Vec<Component>) -> Composer {
        let mut components = components;
        let mut mode_renderer = ModeRenderer::default();
        // let mut tab_renderer = TabRenderer::default();

        match ModeRenderer::new(&config.mode) {
            Ok(m) => mode_renderer = m,
            Err(c) => components = vec![Component::from(c)],
        }
        //
        // match TabRenderer::new(&config.tab) {
        //     Ok(t) => tab_renderer = t,
        //     Err(c) => components = c,
        // }

        Composer {
            components,
            mode_renderer,
            // tab_renderer,
            ..Default::default()
        }
    }

    fn render(&self, component: &Component, cols: usize) -> String {
        match component {
            Component::Text(t) => t.to_string(),
            Component::Style(s) => self.style_renderer.render(s),
            Component::Session => self.session_name.clone(),
            Component::Mode => self.mode_renderer.render(),
            Component::Tab => self.tab_renderer.render(),
            Component::ParseError {
                context,
                layout,
                hl_begin,
                hl_end,
            } => self
                .error_renderer
                .render(cols, context, layout, *hl_begin, *hl_end),
            _ => "".to_string(),
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
        should_render |= self.tab_renderer.update_mode(mode_info.mode, palette);
        should_render |= self.error_renderer.update(palette);
        should_render
    }

    pub fn update_tab(&mut self, tabs: Vec<TabInfo>) -> bool {
        self.tab_renderer.update_tabs(tabs)
    }

    pub fn compose(&self, state: &mut State, cols: usize) -> String {
        let mut res = String::new();

        for c in self.components.iter() {
            res.push_str(&self.render(c, cols))
        }

        res
    }
}
