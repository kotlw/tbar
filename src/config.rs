use std::collections::HashMap;
use zellij_tile::prelude::*;

pub struct Config {
    pub layout: String,
    pub mode: HashMap<InputMode, String>
}

impl Default for Config {
    fn default() -> Config {
        Config {
            layout: "#[bg:black,fg:green,bold] Zellij #[default](#S)#[default] #M ".to_string(),
            mode: HashMap::from([
                (InputMode::Normal, "#[fg:green]NORMAL".to_string()),
                (InputMode::Locked, "#[fg:red]LOCKED".to_string()),
                (InputMode::Resize, "#[fg:orange]RESIZE".to_string()),
                (InputMode::Pane, "#[fg:orange]PANE".to_string()),
                (InputMode::Tab, "#[fg:orange,bold]TAB".to_string()),
                (InputMode::Scroll, "#[fg:orange]SCROLL".to_string()),
                (InputMode::EnterSearch, "#[fg:orange]ENTSEARCH".to_string()),
                (InputMode::Search, "#[fg:orange]SEARCH".to_string()),
                (InputMode::RenameTab, "#[fg:orange]RENAMETAB".to_string()),
                (InputMode::RenamePane, "#[fg:orange]RENAMEPAN".to_string()),
                (InputMode::Session, "#[fg:orange]SESSION".to_string()),
                (InputMode::Move, "#[fg:orange]MOVE".to_string()),
                (InputMode::Prompt, "#[fg:orange]PROMPT".to_string()),
                (InputMode::Tmux, "#[fg:orange]TMUX".to_string()),
            ]),
        }
    }
}
