pub struct ModeStyle {
    normal: String,
    locked: String,
    resize: String,
    pane: String,
    tab: String,
    scroll: String,
    enter_search: String,
    search: String,
    rename_tab: String,
    rename_pane: String,
    session: String,
    r#move: String,
    prompt: String,
    tmux: String,
}

pub struct Config {
    pub layout: String,
    pub mode_style: ModeStyle
}

impl Default for Config {
    fn default() -> Config {
        Config {
            layout: "#[bg:black,fg:green,bold] Zellij #[fg:red](#S)#[default] #M ".to_string(),
            mode_style: ModeStyle {
                normal: "NORMAL".to_string(),
                locked: "LOCKED".to_string(),
                resize: "RESIZE".to_string(),
                pane: "PANE".to_string(),
                tab: "TAB".to_string(),
                scroll: "SCROLL".to_string(),
                enter_search: "ENTER_SEARCH".to_string(),
                search: "SEARCH".to_string(),
                rename_tab: "RENAME TAB".to_string(),
                rename_pane: "RENAME PANE".to_string(),
                session: "SESSION".to_string(),
                r#move: "MOVE".to_string(),
                prompt: "PROMPT".to_string(),
                tmux: "TMUX".to_string(),
            }
        }
    }
}
