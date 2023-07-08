use crate::composer::Component;
use crate::parser::Parser;
use crate::style::StyleRenderer;
use std::collections::HashMap;
use zellij_tile::prelude::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum TabState {
    Inactive,
    Active,
    InactiveSync,
    ActiveSync,
}

#[derive(Default)]
pub struct TabRenderer {
    parsed_tab: HashMap<TabState, Vec<Component>>,

    mode: InputMode,
    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
    style_renderer: StyleRenderer,
}

impl TabRenderer {
    pub fn new(tabs_config: &HashMap<TabState, String>) -> Result<TabRenderer, Vec<Component>> {
        let mut parsed_tab = HashMap::new();

        for (k, v) in tabs_config {
            let tab_components = Parser::new(&v, "[IN").expect_parse("Error parsing tab layout: ");
            parsed_tab.insert(*k, tab_components);

            // Catching errors in modes config and calculating component len.
            if let Some(components) = parsed_tab.get(&k) {
                for c in components {
                    match c {
                        Component::ParseError {
                            hint,
                            layout,
                            hl_begin,
                            hl_end,
                        } => {
                            return Err(vec![Component::ParseError {
                                hint: hint.to_string(),
                                layout: layout.to_string(),
                                hl_begin: *hl_begin,
                                hl_end: *hl_end,
                            }]);
                        }
                        _ => (),
                    }
                }
            }
        }

        Ok(TabRenderer {
            parsed_tab,
            ..Default::default()
        })
    }

    pub fn update_mode(&mut self, mode: InputMode, palette: Palette) -> bool {
        self.style_renderer.update(palette);
        if self.mode != mode {
            self.mode = mode;
            return true;
        }
        false
    }

    pub fn update_tabs(&mut self, tabs: Vec<TabInfo>) -> bool {
        if let Some(active_tab_index) = tabs.iter().position(|t| t.active) {
            // tabs are indexed starting from 1 so we need to add 1
            let active_tab_idx = active_tab_index + 1;
            if self.active_tab_idx != active_tab_idx || self.tabs != tabs {
                return true;
            }
            self.active_tab_idx = active_tab_idx;
            self.tabs = tabs;
        } else {
            eprintln!("Could not find active tab.");
        }
        false
    }

    pub fn render(&self) -> String {
        "{TABS}".to_string()
    }
}
