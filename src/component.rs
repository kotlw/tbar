use crate::config::ModeStyle;
use zellij_tile::prelude::*;
use crate::tokenizer;

pub trait Component {
    fn render(&self) -> &String;
}

struct StyledText {
    value: String,
    len: usize,
}

impl Component for StyledText {
    fn render(&self) -> &String {
        &self.value
    }
}

impl StyledText {
    fn from_token(token: tokenizer::Token) -> StyledText {
        let mut len = 0;
        if token.kind != tokenizer::Kind::Style {
            len = token.value.chars().count();
        }
        StyledText { value: token.value, len }
    }

    fn push_token(&mut self, token: tokenizer::Token) {
        let mut len = 0;
        if token.kind != tokenizer::Kind::Style {
            len = token.value.chars().count();
        }
        self.value.push_str(&token.value);
        self.len += len;
    }

    fn len(&self) -> usize {
        self.len
    }
}


#[derive(Default)]
struct Mode {
    mode: InputMode,
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

impl Component for Mode {
    fn render(&self) -> &String {
        match self.mode {
            InputMode::Normal => &self.normal,
            InputMode::Locked => &self.locked,
            InputMode::Resize => &self.resize,
            InputMode::Pane => &self.pane,
            InputMode::Tab => &self.tab,
            InputMode::Scroll => &self.scroll,
            InputMode::EnterSearch => &self.enter_search,
            InputMode::Search => &self.search,
            InputMode::RenameTab => &self.rename_tab,
            InputMode::RenamePane => &self.rename_pane,
            InputMode::Session => &self.session,
            InputMode::Move => &self.r#move,
            InputMode::Prompt => &self.prompt,
            InputMode::Tmux => &self.tmux,
        }
    }
}

impl Mode {
    fn new(mode: InputMode) -> Mode {
        Mode { mode, ..Default::default() }
    }
    fn set_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }
}
