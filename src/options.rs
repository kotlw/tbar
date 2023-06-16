pub struct Options {
    pub separator: String,
    pub transparent_bg: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            separator: "".to_string(),
            transparent_bg: true 
        }
    }
}
