use crate::composer::Component;
use crate::parser::Parser;
use zellij_tile::prelude::*;

#[derive(Default)]
pub struct TabRenderer {
    active_tab: Vec<Component>,
    tab: Vec<Component>,

    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
}

impl TabRenderer {
    pub fn new(active_tab: String, tab: String) -> Result<TabRenderer, Vec<Component>> {
        let active_tab_components =
            Parser::new(&active_tab, "[IW").expect_parse("Error parsing mode: ");

        Err(vec![Component::Text("he".to_string())])
    }

    pub fn update(&mut self, tabs: Vec<TabInfo>) -> bool {
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
