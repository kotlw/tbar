pub struct Options {
    pub separator: String,
    pub transparent_bg: bool,
    pub line: String
}

impl Default for Options {
    fn default() -> Options {
        Options {
            separator: "".to_string(),
            transparent_bg: true,
            line: " {{green}}Zellij{{default}} ({{S}}) tabs)) ".to_string()
        }
    }
}
