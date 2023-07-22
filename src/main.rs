use std::cmp::{max, min};
use std::collections::HashMap;

use zellij_tile::prelude::*;

mod config;
mod parser;
use crate::config::Config;
use crate::parser::{Color, Component, ParseError, Parser, Style};

type ModeLayouts<'a> = HashMap<InputMode, &'a str>;
type TabLayouts<'a> = HashMap<TabPartState, &'a str>;
type SwapLayouts<'a> = HashMap<SwapLayoutState, &'a str>;

type ModeComponents = HashMap<InputMode, Vec<Component>>;
type TabComponents = HashMap<TabPartState, Vec<Component>>;
type SwapComponents = HashMap<SwapLayoutState, Vec<Component>>;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum TabPartState {
    Inactive,
    Active,
    InactiveSync,
    ActiveSync,
    LeftMoreTabs,
    RightMoreTabs,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum SwapLayoutState {
    NonDirty,
    Dirty,
}

#[derive(Default, Clone)]
struct RenderedTabPart {
    index: usize,
    value: String,
    len: usize,
}

#[derive(Default)]
struct State {
    layout_components: Vec<Component>,
    mode_components: ModeComponents,
    tab_components: TabComponents,
    swap_components: SwapComponents,

    mode_info: ModeInfo,
    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
    mouse_click_pos: usize,
    should_change_tab: bool,
    cols: usize,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        let cfg = Config::default();

        self.layout_components = match Self::parse_layout(&cfg.layout) {
            Ok(c) => c,
            Err(e) => Self::prepare_error("Error parsing mode: ", e),
        };

        match Self::parse_mode_layouts(&cfg.mode_layouts) {
            Ok(c) => self.mode_components = c,
            Err(e) => self.layout_components = Self::prepare_error("Error parsing mode: ", e),
        }

        match Self::parse_tab_layouts(&cfg.tab_layouts) {
            Ok(c) => self.tab_components = c,
            Err(e) => self.layout_components = Self::prepare_error("Error parsing tab: ", e),
        }

        match Self::parse_swap_layouts(&cfg.swap_layouts) {
            Ok(c) => self.swap_components = c,
            Err(e) => self.layout_components = Self::prepare_error("Error parsing swap: ", e),
        }

        set_selectable(false);
        subscribe(&[
            EventType::ModeUpdate,
            EventType::TabUpdate,
            EventType::Mouse,
        ]);
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
            Event::Mouse(me) => match me {
                Mouse::LeftClick(_, col) => {
                    if self.mouse_click_pos != col {
                        should_render = true;
                        self.should_change_tab = true;
                    }
                    self.mouse_click_pos = col;
                }
                Mouse::ScrollUp(_) => {
                    should_render = true;
                    switch_tab_to(min(self.active_tab_idx + 2, self.tabs.len()) as u32);
                }
                Mouse::ScrollDown(_) => {
                    should_render = true;
                    switch_tab_to(max(self.active_tab_idx.saturating_sub(2), 1) as u32);
                }
                _ => {}
            },
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
        self.cols = cols;

        for (i, component) in self.layout_components.iter().enumerate() {
            if let Component::Spacer = component {
                res.push("".to_string());
                spacer_pos.push(i);
                continue;
            }
            let (rendered, len) = self.render_layout_component(component, cols_left);
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
        self.should_change_tab = false;
    }
}

impl State {
    fn parse_layout(layout: &str) -> Result<Vec<Component>, ParseError> {
        let allowed_specials = vec![
            Component::Spacer,
            Component::Style(Style::Default),
            Component::Session,
            Component::Mode,
            Component::TabBar,
            Component::SwapLayout,
        ];
        Ok(Parser::new(layout, allowed_specials).parse()?)
    }

    fn parse_mode_layouts(layouts: &ModeLayouts) -> Result<ModeComponents, ParseError> {
        let mut res = HashMap::new();

        for (k, v) in layouts {
            let components = Parser::new(&v, vec![Component::Style(Style::Default)]).parse()?;
            res.insert(*k, components);
        }

        Ok(res)
    }

    fn parse_tab_layouts(layouts: &TabLayouts) -> Result<TabComponents, ParseError> {
        let mut res = HashMap::new();

        for (k, v) in layouts {
            let mut allowed_specials = vec![Component::Style(Style::Default), Component::Index];
            if !matches!(k, TabPartState::LeftMoreTabs) && !matches!(k, TabPartState::RightMoreTabs)
            {
                allowed_specials.push(Component::Name);
            }

            let components = Parser::new(&v, allowed_specials).parse()?;
            res.insert(*k, components);
        }

        Ok(res)
    }

    fn parse_swap_layouts(layouts: &SwapLayouts) -> Result<SwapComponents, ParseError> {
        let mut res = HashMap::new();

        for (k, v) in layouts {
            let allowed_specials = vec![Component::Style(Style::Default), Component::Name];
            let components = Parser::new(&v, allowed_specials).parse()?;
            res.insert(*k, components);
        }

        Ok(res)
    }

    fn prepare_error(aditional_context: &str, e: ParseError) -> Vec<Component> {
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
        let layout_end = min(layout_len, hl_end + offset);

        // Setup layout wrapping strings
        let wrap_left = if layout_begin > 0 { "..." } else { "^" };
        let wrap_right = if layout_end < layout_len { "..." } else { "$" };

        // Squeeze highlighted text if needed.
        let squeeze_size = (hl_len + layout_wrap_len).saturating_sub(cols_left);
        let hl_end_squeezed = max(hl_begin, hl_end.saturating_sub(squeeze_size));
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

        for c in &self.mode_components[&self.mode_info.mode] {
            let (rendered, curr_len) = self.render_layout_component(c, cols_left);
            res.push_str(&rendered);
            len += curr_len;
        }

        if cols_left < len {
            ("".to_string(), 0)
        } else {
            (res, len)
        }
    }

    fn render_tab_part(
        &self,
        tab_part_state: TabPartState,
        index: usize,
        name: &str,
    ) -> RenderedTabPart {
        let mut render_tab_name = name.clone();
        let mut value = String::new();
        let mut len = 0;

        if self.tab_components[&tab_part_state]
            .iter()
            .any(|x| matches!(x, Component::Index))
        {
            render_tab_name = "Tab"
        }

        for c in &self.tab_components[&tab_part_state] {
            let (rendered, curr_len) = match c {
                Component::Text(t) => self.render_text(&t, usize::MAX),
                Component::Style(s) => self.render_style(&s),
                Component::Index => self.render_text(&index.to_string(), usize::MAX),
                Component::Name => self.render_text(render_tab_name, usize::MAX),
                _ => self.render_text("{unparsed}", usize::MAX),
            };
            value.push_str(&rendered);
            len += curr_len;
        }

        RenderedTabPart { index, value, len }
    }

    fn get_tab_parts(&self) -> Vec<RenderedTabPart> {
        let mut res = Vec::new();

        for (i, t) in self.tabs.iter().enumerate() {
            let layout_key = match (t.active, t.is_sync_panes_active) {
                (true, true) => TabPartState::ActiveSync,
                (false, true) => TabPartState::InactiveSync,
                (true, false) => TabPartState::Active,
                (false, false) => TabPartState::Inactive,
            };
            res.push(self.render_tab_part(layout_key, i + 1, &t.name));
        }

        res
    }

    fn change_active_tab(&self, tab_parts: &Vec<RenderedTabPart>, cols_left: usize) {
        let mut len_cnt = self.cols.saturating_sub(cols_left);
        for part in tab_parts {
            if self.mouse_click_pos >= len_cnt && self.mouse_click_pos < len_cnt + part.len {
                switch_tab_to(part.index as u32);
            }
            len_cnt += part.len;
        }
    }

    // TODO: this is huge
    fn render_tab_bar(&self, cols_left: usize) -> (String, usize) {
        let mut tab_parts = self.get_tab_parts();
        let mut before_active_tab_count = self.active_tab_idx;
        let mut after_active_tab_count = tab_parts.len().saturating_sub(self.active_tab_idx + 1);
        let mut collapsed_left_count = 0;
        let mut collapsed_right_count = 0;
        let mut collapsed_left = RenderedTabPart::default();
        let mut collapsed_right = RenderedTabPart::default();

        let (res, len, parts) = loop {
            let mut tab_parts_with_collapsed = tab_parts.clone();
            if tab_parts.len() > 0 {
                collapsed_left.index = tab_parts.first().unwrap().index.saturating_sub(1);
                collapsed_right.index = tab_parts.last().unwrap().index + 1;
            }
            tab_parts_with_collapsed.insert(0, collapsed_left.clone());
            tab_parts_with_collapsed.push(collapsed_right.clone());

            // Break the loop when it fits cols_left
            let tab_parts_total_len = tab_parts_with_collapsed.iter().map(|x| x.len).sum();
            if tab_parts_total_len <= cols_left {
                let iter = tab_parts_with_collapsed.iter();
                let res = iter.map(|x| x.value.to_string()).collect::<String>();
                break (res, tab_parts_total_len, tab_parts_with_collapsed);
            }

            // return empty if cols_left is less than an active tab length
            if tab_parts.len() == 1 && tab_parts.first().unwrap().len > cols_left {
                break ("".to_string(), 0, tab_parts_with_collapsed);
            }

            // remove from tab_parts and increment collapsed tabs count
            if before_active_tab_count >= after_active_tab_count && before_active_tab_count != 0 {
                before_active_tab_count = before_active_tab_count.saturating_sub(1);
                collapsed_left_count += 1;
                tab_parts.remove(0);
                collapsed_left =
                    self.render_tab_part(TabPartState::LeftMoreTabs, collapsed_left_count, "");
            } else if after_active_tab_count != 0 {
                after_active_tab_count = after_active_tab_count.saturating_sub(1);
                collapsed_right_count += 1;
                tab_parts.pop();
                collapsed_right =
                    self.render_tab_part(TabPartState::RightMoreTabs, collapsed_right_count, "");
            } else {
                collapsed_left = RenderedTabPart::default();
                collapsed_right = RenderedTabPart::default();
            };
        };

        if self.should_change_tab {
            self.change_active_tab(&parts, cols_left)
        }

        (res, len)
    }

    fn render_swap_layout_part(&self, name: String, is_dirty: bool) -> (String, usize) {
        let mut res = String::new();
        let mut len = 0;
        let key = match is_dirty {
            true => SwapLayoutState::Dirty,
            false => SwapLayoutState::NonDirty,
        };

        for c in &self.swap_components[&key] {
            let (rendered, curr_len) = match c {
                Component::Text(t) => self.render_text(&t, usize::MAX),
                Component::Style(s) => self.render_style(&s),
                Component::Name => self.render_text(&name, usize::MAX),
                _ => self.render_text("{unparsed}", usize::MAX),
            };
            res.push_str(&rendered);
            len += curr_len;
        }

        (res, len)
    }

    fn render_swap_layout(&self, cols_left: usize) -> (String, usize) {
        if let Some(active_tab) = &self.tabs.iter().nth(self.active_tab_idx) {
            let (rendered, len) = match &active_tab.active_swap_layout_name {
                Some(n) => {
                    self.render_swap_layout_part(n.to_string(), active_tab.is_swap_layout_dirty)
                }
                None => ("".to_string(), 0),
            };

            if len > cols_left {
                ("".to_string(), 0)
            } else {
                (rendered, len)
            }
        } else {
            ("".to_string(), 0)
        }
    }

    fn render_layout_component(&self, component: &Component, cols_left: usize) -> (String, usize) {
        match component {
            Component::Text(t) => self.render_text(t, cols_left),
            Component::Style(s) => self.render_style(s),
            Component::Session => self.render_session(cols_left),
            Component::Mode => self.render_mode(cols_left),
            Component::TabBar => self.render_tab_bar(cols_left),
            Component::SwapLayout => self.render_swap_layout(cols_left),
            Component::LayoutHighlight {
                layout,
                hl_begin,
                hl_end,
            } => self.render_layout_highlight(cols_left, layout.to_string(), *hl_begin, *hl_end),
            _ => self.render_text("{unparsed}", cols_left),
        }
    }
}
