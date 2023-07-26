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
            layout: "#[fg:white,bg:black,bold] Zellij (#S) #M#T#[bg:black]#_#L  ",
            mode_layouts: HashMap::from([
                (InputMode::Normal, "#[bold,fg:green] NORMAL #[default]"),
                (InputMode::Locked, "#[bold,fg:red] LOCKED #[default]"),
                (InputMode::Resize, "#[bold,fg:orange] RESIZE #[default]"),
                (InputMode::Pane, "#[bold,fg:orange]  PANE  #[default]"),
                (InputMode::Tab, "#[bold,fg:orange]  TAB   #[default]"),
                (InputMode::Scroll, "#[bold,fg:orange] SCROLL #[default]"),
                (
                    InputMode::EnterSearch,
                    "#[bold,fg:orange]ENTSEARCH#[default]",
                ),
                (InputMode::Search, "#[bold,fg:orange] SEARCH #[default]"),
                (InputMode::RenameTab, "#[bold,fg:orange]RENAMETAB#[default]"),
                (
                    InputMode::RenamePane,
                    "#[bold,fg:orange]RENAMEPANE#[default]",
                ),
                (InputMode::Session, "#[bold,fg:orange]SESSION #[default]"),
                (InputMode::Move, "#[bold,fg:orange]  MOVE  #[default]"),
                (InputMode::Prompt, "#[bold,fg:orange] PROMPT #[default]"),
                (InputMode::Tmux, "#[bold,fg:orange]  TMUX  #[default]"),
            ]),
            tab_layouts: HashMap::from([
                (
                    TabPartState::Inactive,
                    "#[bg:white,fg:black,bold] #N #[bg:black,fg:white]#[default]",
                ),
                (
                    TabPartState::Active,
                    "#[bg:green,fg:black,bold] #N #[bg:black,fg:green]#[default]",
                ),
                (
                    TabPartState::InactiveSync,
                    "#[bg:white,fg:black,bold] #N (Sync) #[bg:black,fg:white]#[default]",
                ),
                (
                    TabPartState::ActiveSync,
                    "#[bg:green,fg:black,bold] #N (Sync) #[bg:black,fg:green]#[default]",
                ),
                (
                    TabPartState::LeftMoreTabs,
                    "#[bg:orange,fg:black,bold]#[fg:white] ← +#I #[bg:black,fg:orange]#[default]",
                ),
                (
                    TabPartState::RightMoreTabs,
                    "#[bg:orange,fg:black,bold]#[fg:white] +#I → #[bg:black,fg:orange]#[default]",
                ),
            ]),
            swap_layouts: HashMap::from([
                (
                    SwapLayoutState::NonDirty,
                    "#[bg:green,fg:black,bold] #N #[bg:black,fg:green]",
                ),
                (
                    SwapLayoutState::Dirty,
                    "#[bg:white,fg:black,bold] #N #[bg:black,fg:white]",
                ),
            ]),
        }
    }
}
