#[derive(Debug)]
struct Options {
    separator: String,
    background: String,
}

#[derive(Debug)]
struct Config {
    options: Options,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            options: Options {
                separator: "1".to_string(),
                background: "tr".to_string(),
            }
        }
    }
}
