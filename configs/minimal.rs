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
            layout: "#T#_#L#M",
            mode_layouts: HashMap::from([
                (InputMode::Normal, "#[fg:green]  #[default]"),
                (InputMode::Locked, "#[fg:red]  #[default]"),
                (InputMode::Resize, "#[fg:orange] 󰙖 #[default]"),
                (InputMode::Pane, "#[bold,fg:orange]  #[default]"),
                (InputMode::Tab, "#[bold,fg:orange] 󰓩 #[default]"),
                (InputMode::Scroll, "#[fg:orange]  #[default]"),
                (InputMode::EnterSearch, "#[fg:orange]  #[default]"),
                (InputMode::Search, "#[fg:orange]  #[default]"),
                (InputMode::RenameTab, "#[fg:orange] 󰇘 #[default]"),
                (InputMode::RenamePane, "#[fg:orange] 󰇘 #[default]"),
                (InputMode::Session, "#[fg:orange]  #[default]"),
                (InputMode::Move, "#[fg:orange] 󰆾 #[default]"),
                (InputMode::Prompt, "#[fg:orange] P #[default]"),
                (InputMode::Tmux, "#[fg:orange] T #[default]"),
            ]),
            tab_layouts: HashMap::from([
                (TabPartState::Inactive, " #I #N "),
                (TabPartState::Active, "#[fg:green] #I #N #[default]"),
                (TabPartState::InactiveSync, " #I #N 󰓦 #[default]"),
                (TabPartState::ActiveSync, "#[fg:green] #I #N 󰓦 #[default]"),
                (TabPartState::LeftMoreTabs, "#[fg:orange] ← +#I #[default]"),
                (TabPartState::RightMoreTabs, "#[fg:orange] +#I → #[default]"),
            ]),
            swap_layouts: HashMap::from([
                (SwapLayoutState::NonDirty, "#[fg:green]#N#[default]"),
                (SwapLayoutState::Dirty, "#[default]#N#[default]"),
            ]),
        }
    }
}
