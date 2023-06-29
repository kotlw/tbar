pub struct Options {
    pub layout: String
}

impl Default for Options {
    fn default() -> Options {
        Options {
            layout: " Zellij #[green](#S)#[default] #M ".to_string()
        }
    }
}
