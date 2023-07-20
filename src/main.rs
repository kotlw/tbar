use std::cmp;
use std::collections::HashMap;

use zellij_tile::prelude::*;

mod config;
mod parser;
use crate::config::Config;
use crate::parser::{Color, Component, ParseError, Parser, Style};

#[derive(Default)]
struct State {
    layout_components: Vec<Component>,
    modes_components: HashMap<InputMode, Vec<Component>>,
    tabs_components: HashMap<TabLayout, Vec<Component>>,

    mode_info: ModeInfo,
    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        let cfg = Config::default();

        self.layout_components = Parser::new(
            &cfg.layout,
            vec![
                Component::Spacer,
                Component::Style(Style::Default),
                Component::Session,
                Component::Mode,
                Component::Tab,
            ],
        )
        .parse()
        .unwrap_or_else(|e| self.prepare_error_components("Error parsing layout: ", e));

        match self.parse_modes(&cfg.mode) {
            Ok(c) => self.modes_components = c,
            Err(e) => {
                self.layout_components = self.prepare_error_components("Error parsing mode: ", e)
            }
        }

        match self.parse_tabs(&cfg.tab) {
            Ok(c) => self.tabs_components = c,
            Err(e) => {
                self.layout_components = self.prepare_error_components("Error parsing tab: ", e)
            }
        }

        // self.composer = Composer::new(&cfg, components);
        set_selectable(false);
        subscribe(&[EventType::ModeUpdate, EventType::TabUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    should_render = true;
                }
                self.mode_info = mode_info
            }
            Event::TabUpdate(tabs) => {
                if let Some(active_tab_index) = tabs.iter().position(|t| t.active) {
                    let active_tab_idx = active_tab_index;
                    if self.active_tab_idx != active_tab_idx || self.tabs != tabs {
                        should_render = true;
                    }
                    self.active_tab_idx = active_tab_idx;
                    self.tabs = tabs;
                } else {
                    eprintln!("Could not find active tab.");
                }
            }
            // Event::TabUpdate(tabs) => self.composer.update_tab(tabs),
            _ => {
                eprintln!("Got unrecognized event: {:?}", event);
            }
        };
        should_render
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        let mut res = Vec::new();
        let mut spacer_pos = Vec::new();
        let mut cols_left = cols;

        for (i, component) in self.layout_components.iter().enumerate() {
            if let Component::Spacer = component {
                res.push("".to_string());
                spacer_pos.push(i);
                continue;
            }
            let (rendered, len) = self.render_component(component, cols_left);
            cols_left = cols_left.saturating_sub(len);
            res.push(rendered);
        }

        if spacer_pos.len() == 0 {
            spacer_pos.push(res.len());
            res.push("".to_string());
        }
        let spacer_len = (cols_left + spacer_pos.len() - 1) / spacer_pos.len();

        for i in spacer_pos {
            let n = if cols_left > spacer_len {
                spacer_len
            } else {
                cols_left
            };
            cols_left = cols_left.saturating_sub(spacer_len);
            res[i] = " ".repeat(n);
        }

        print!("{}", res.join(""));
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum TabLayout {
    Inactive,
    Active,
    InactiveSync,
    ActiveSync,
    LeftMoreTabs,
    RightMoreTabs,
}

impl State {
    fn parse_modes(
        &self,
        modes_config: &HashMap<InputMode, String>,
    ) -> Result<HashMap<InputMode, Vec<Component>>, ParseError> {
        let mut res = HashMap::new();
        for (k, v) in modes_config {
            let components = Parser::new(&v, vec![Component::Style(Style::Default)]).parse()?;
            res.insert(*k, components);
        }
        Ok(res)
    }

    fn parse_tabs(
        &self,
        tabs_config: &HashMap<TabLayout, String>,
    ) -> Result<HashMap<TabLayout, Vec<Component>>, ParseError> {
        let mut res = HashMap::new();
        for (k, v) in tabs_config {
            let mut allowed_specials = vec![Component::Style(Style::Default), Component::Index];
            if !matches!(k, TabLayout::LeftMoreTabs) && !matches!(k, TabLayout::RightMoreTabs) {
                allowed_specials.push(Component::Name);
            }
            let components = Parser::new(&v, allowed_specials).parse()?;
            res.insert(*k, components);
        }
        Ok(res)
    }

    fn prepare_error_components(&self, aditional_context: &str, e: ParseError) -> Vec<Component> {
        vec![
            Component::Style(Style::Bg(Color::Red)),
            Component::Style(Style::Fg(Color::Black)),
            Component::Text(aditional_context.to_string()),
            Component::Text(e.context),
            Component::LayoutHighlight {
                layout: e.layout,
                hl_begin: e.hl_begin,
                hl_end: e.hl_end,
            },
        ]
    }

    fn get_ansi_color(&self, color: &Color) -> ansi_term::Color {
        let palette = self.mode_info.style.colors;
        let p = match color {
            Color::Black => palette.black,
            Color::Red => palette.red,
            Color::Green => palette.green,
            Color::Yellow => palette.yellow,
            Color::Blue => palette.blue,
            Color::Magenta => palette.magenta,
            Color::Cyan => palette.cyan,
            Color::White => palette.white,
            Color::Orange => palette.orange,
            Color::Gray => palette.gray,
            Color::Purple => palette.purple,
            Color::Gold => palette.gold,
            Color::Silver => palette.silver,
            Color::Pink => palette.pink,
            Color::Brown => palette.brown,
        };

        match p {
            PaletteColor::Rgb((r, g, b)) => ansi_term::Color::RGB(r, g, b),
            PaletteColor::EightBit(color) => ansi_term::Color::Fixed(color),
        }
    }

    fn render_style(&self, style: &Style) -> (String, usize) {
        let s = ansi_term::Style::new();
        let res = match style {
            Style::Fg(c) => s.fg(self.get_ansi_color(c)).prefix().to_string(),
            Style::Bg(c) => s.on(self.get_ansi_color(c)).prefix().to_string(),
            Style::Bold => s.bold().prefix().to_string(),
            Style::Default => s.on(ansi_term::Color::Fixed(0)).suffix().to_string(),
        };
        (res, 0)
    }

    fn render_session(&self, cols_left: usize) -> (String, usize) {
        let opt = self.mode_info.session_name.clone();
        self.render_text(&opt.unwrap_or("".to_string()), cols_left)
    }

    fn render_text(&self, text: &str, cols_left: usize) -> (String, usize) {
        if cols_left < text.chars().count() {
            (text.chars().take(cols_left).collect(), cols_left)
        } else {
            (text.to_string(), text.chars().count())
        }
    }

    fn render_layout_highlight(
        &self,
        cols_left: usize,
        layout: String,
        hl_begin: usize,
        hl_end: usize,
    ) -> (String, usize) {
        // Func constants
        let (bg_color, _) = self.render_style(&Style::Bg(Color::Red));
        let (hl_color, _) = self.render_style(&Style::Bg(Color::Yellow));
        let styles_len = bg_color.chars().count() + hl_color.chars().count();
        let layout_wrap_len = 6;
        let layout_len = layout.chars().count();
        let hl_len = hl_end.saturating_sub(hl_begin);

        // Calculate layout window beginning and end
        let offset = cols_left.saturating_sub(hl_len + layout_wrap_len) / 2;
        let layout_begin = hl_begin.saturating_sub(offset);
        let layout_end = cmp::min(layout_len, hl_end + offset);

        // Setup layout wrapping strings
        let wrap_left = if layout_begin > 0 { "..." } else { "^" };
        let wrap_right = if layout_end < layout_len { "..." } else { "$" };

        // Squeeze highlighted text if needed.
        let squeeze_size = (hl_len + layout_wrap_len).saturating_sub(cols_left);
        let hl_end_squeezed = cmp::max(hl_begin, hl_end.saturating_sub(squeeze_size));
        if hl_end_squeezed <= hl_begin {
            return ("......".chars().take(cols_left).collect(), cols_left);
        };

        // handle string slice with unicode chars
        let layout_unicode_slice = |start, end| {
            let l = layout.chars().collect::<Vec<_>>();
            l.get(start..end).unwrap().iter().collect::<String>()
        };
        let layout_before_hl = layout_unicode_slice(layout_begin, hl_begin);
        let layout_hl = layout_unicode_slice(hl_begin, hl_end_squeezed);
        let layout_after_hl = layout_unicode_slice(hl_end, layout_end);
        let res = format!("{wrap_left}{layout_before_hl}{hl_color}{layout_hl}{bg_color}{layout_after_hl}{wrap_right}");

        (res.to_string(), res.chars().count() - styles_len)
    }

    fn render_mode(&self, cols_left: usize) -> (String, usize) {
        let mut res = String::new();
        let mut len = 0;

        for c in &self.modes_components[&self.mode_info.mode] {
            let (rendered, curr_len) = self.render_component(c, cols_left);
            res.push_str(&rendered);
            len += curr_len;
        }

        let (default_style, _) = &self.render_style(&Style::Default);
        res.push_str(default_style);

        if cols_left < len {
            ("".to_string(), 0)
        } else {
            (res, len)
        }
    }

    // fn render_inactive_tab(&self, tab: TabInfo) -> (String, usize) {}

    // fn render_tab(&self, cols_left: usize) -> (String, usize) {
    //     let mut tabs = self.tabs.clone();
    //     let mut tabs_after_active = tabs.split_off(self.active_tab_idx);
    //     let mut tabs_before_active = tabs;
    //
    //     todo!()
    // }

    fn render_component(&self, component: &Component, cols_left: usize) -> (String, usize) {
        match component {
            Component::Text(t) => self.render_text(t, cols_left),
            Component::Style(s) => self.render_style(s),
            Component::Session => self.render_session(cols_left),
            Component::Mode => self.render_mode(cols_left),
            Component::LayoutHighlight {
                layout,
                hl_begin,
                hl_end,
            } => self.render_layout_highlight(cols_left, layout.to_string(), *hl_begin, *hl_end),
            _ => self.render_text("{unparsed}", cols_left),
        }
    }
}
