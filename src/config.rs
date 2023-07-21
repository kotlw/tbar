use crate::TabPartState;
use std::collections::HashMap;
use zellij_tile::prelude::*;

pub struct Config<'a> {
    pub layout: &'a str,
    pub mode_layouts: HashMap<InputMode, &'a str>,
    pub tab_layouts: HashMap<TabPartState, &'a str>,
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            layout: "#[fg:white,bold] Zellij #S #[default] #M | #T#_",
            mode_layouts: HashMap::from([
                (InputMode::Normal, "#[fg:green]NORMAL#[default]"),
                (InputMode::Locked, "#[fg:red]LOCKED#[default]"),
                (InputMode::Resize, "#[fg:orange]RESIZE#[default]"),
                (InputMode::Pane, "#[fg:orange]PANE#[default]"),
                (InputMode::Tab, "#[fg:orange,bold]TAB#[default]"),
                (InputMode::Scroll, "#[fg:orange]SCROLL#[default]"),
                (InputMode::EnterSearch, "#[fg:orange]ENTSEARCH#[default]"),
                (InputMode::Search, "#[fg:orange]SEARCH#[default]"),
                (InputMode::RenameTab, "#[fg:orange]RENAMETAB#[default]"),
                (InputMode::RenamePane, "#[fg:orange]RENAMEPAN#[default]"),
                (InputMode::Session, "#[fg:orange]SESSION#[default]"),
                (InputMode::Move, "#[fg:orange]MOVE#[default]"),
                (InputMode::Prompt, "#[fg:orange]PROMPT#[default]"),
                (InputMode::Tmux, "#[fg:orange]TMUX#[default]"),
            ]),
            tab_layouts: HashMap::from([
                (TabPartState::Inactive, "#[bg:white,fg:black] #I #N #[default]"),
                (TabPartState::Active, "#[bg:green,fg:black] #I #N #[default]"),
                (TabPartState::InactiveSync, "#[bg:white,fg:black] #I #N (Sync) #[default]"),
                (TabPartState::ActiveSync, "#[bg:green,fg:black] #I #N (Sync) #[default]"),
                (TabPartState::LeftMoreTabs, "#[bg:orange,fg:black] ← +#I #[default]"),
                (TabPartState::RightMoreTabs, "#[bg:orange,fg:black] +#I → #[default]"),
            ]),
        }
    }
}
