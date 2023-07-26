use crate::{SwapLayoutState, TabPartState};
use std::collections::HashMap;
use zellij_tile::prelude::*;

pub struct Config<'a> {
    pub layout: &'a str,
    pub mode_layouts: HashMap<InputMode, &'a str>,
    pub tab_layouts: HashMap<TabPartState, &'a str>,
    pub swap_layouts: HashMap<SwapLayoutState, &'a str>,
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            layout: "",
            mode_layouts: HashMap::from([
                (InputMode::Normal, ""),
                (InputMode::Locked, ""),
                (InputMode::Resize, ""),
                (InputMode::Pane, ""),
                (InputMode::Tab, ""),
                (InputMode::Scroll, ""),
                (InputMode::EnterSearch, ""),
                (InputMode::Search, ""),
                (InputMode::RenameTab, ""),
                (InputMode::RenamePane, ""),
                (InputMode::Session, ""),
                (InputMode::Move, ""),
                (InputMode::Prompt, ""),
                (InputMode::Tmux, ""),
            ]),
            tab_layouts: HashMap::from([
                (TabPartState::Inactive, ""),
                (TabPartState::Active, ""),
                (TabPartState::InactiveSync, ""),
                (TabPartState::ActiveSync, ""),
                (TabPartState::LeftMoreTabs, ""),
                (TabPartState::RightMoreTabs, ""),
            ]),
            swap_layouts: HashMap::from([
                (SwapLayoutState::NonDirty, ""),
                (SwapLayoutState::Dirty, ""),
            ]),
        }
    }
}
