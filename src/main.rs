use std::cmp;
use std::collections::HashMap;

use zellij_tile::prelude::*;

mod config;
mod parser;

use crate::config::Config;
use crate::parser::{Component, Parser};

#[derive(Default)]
struct State {
    components: Vec<Component>,
    modes: HashMap<InputMode, Vec<Component>>,

    mode_info: ModeInfo,
    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        // it will be possible to parse zellij config in the future
        let cfg = Config::default();
        self.components = self.prepare_components(&cfg.layout);
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
                    // tabs are indexed starting from 1 so we need to add 1
                    let active_tab_idx = active_tab_index + 1;
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

        for (i, component) in self.components.iter().enumerate() {
            if let Component::Spacer = component {
                res.push("".to_string());
                spacer_pos.push(i);
                continue;
            }
            let (rendered, len) = self.render_component(component, cols_left);
            cols_left = cols_left.saturating_sub(len);
            res.push(rendered);
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
pub enum TabState {
    Inactive,
    Active,
    InactiveSync,
    ActiveSync,
}

impl State {
    fn prepare_components(&self, layout: &str) -> Vec<Component> {
        let mut res = Parser::new(layout, "_[SMT")
            .parse()
            .unwrap_or_else(|e| self.prepare_error_components("Error parsing layout: ", e));

        if !res.iter().any(|c| matches!(c, Component::Spacer)) {
            res.push(parser::Component::Spacer);
        }
        res
    }

    fn prepare_error_components(
        &self,
        aditional_context: &str,
        e: parser::ParseError,
    ) -> Vec<Component> {
        vec![
            Component::Style(parser::Style::Bg(parser::Color::Red)),
            Component::Style(parser::Style::Fg(parser::Color::Black)),
            Component::Text(aditional_context.to_string()),
            Component::Text(e.context),
            Component::LayoutHighlight {
                layout: e.layout,
                hl_begin: e.hl_begin,
                hl_end: e.hl_end,
            },
        ]
    }

    fn get_ansi_color(&self, color: &parser::Color) -> ansi_term::Color {
        let palette = self.mode_info.style.colors;
        let p = match color {
            parser::Color::Black => palette.black,
            parser::Color::Red => palette.red,
            parser::Color::Green => palette.green,
            parser::Color::Yellow => palette.yellow,
            parser::Color::Blue => palette.blue,
            parser::Color::Magenta => palette.magenta,
            parser::Color::Cyan => palette.cyan,
            parser::Color::White => palette.white,
            parser::Color::Orange => palette.orange,
            parser::Color::Gray => palette.gray,
            parser::Color::Purple => palette.purple,
            parser::Color::Gold => palette.gold,
            parser::Color::Silver => palette.silver,
            parser::Color::Pink => palette.pink,
            parser::Color::Brown => palette.brown,
        };

        match p {
            PaletteColor::Rgb((r, g, b)) => ansi_term::Color::RGB(r, g, b),
            PaletteColor::EightBit(color) => ansi_term::Color::Fixed(color),
        }
    }

    fn render_style(&self, style: &parser::Style) -> (String, usize) {
        let s = ansi_term::Style::new();
        let res = match style {
            parser::Style::Fg(c) => s.fg(self.get_ansi_color(c)).prefix().to_string(),
            parser::Style::Bg(c) => s.on(self.get_ansi_color(c)).prefix().to_string(),
            parser::Style::Bold => s.bold().prefix().to_string(),
            parser::Style::Default => s.on(ansi_term::Color::Fixed(0)).suffix().to_string(),
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
        let (bg_color, _) = self.render_style(&parser::Style::Bg(parser::Color::Red));
        let (hl_color, _) = self.render_style(&parser::Style::Bg(parser::Color::Yellow));
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

        let layout_before_hl = &layout[layout_begin..hl_begin];
        let layout_hl = &layout[hl_begin..hl_end_squeezed];
        let layout_after_hl = &layout[hl_end..layout_end];
        let res = format!("{wrap_left}{layout_before_hl}{hl_color}{layout_hl}{bg_color}{layout_after_hl}{wrap_right}");

        (res.to_string(), res.chars().count() - styles_len)
    }

    fn render_component(&self, component: &Component, cols_left: usize) -> (String, usize) {
        match component {
            Component::Text(t) => self.render_text(t, cols_left),
            Component::Style(s) => self.render_style(s),
            Component::Session => self.render_session(cols_left),
            Component::LayoutHighlight {
                layout,
                hl_begin,
                hl_end,
            } => self.render_layout_highlight(cols_left, layout.to_string(), *hl_begin, *hl_end),
            _ => ("{unparsed}".to_string(), 10),
        }
    }
}
