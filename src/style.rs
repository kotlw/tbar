pub fn apply(style: &String) -> String {
    let mut res = String::new();
    for s in style.split(',') {
        res.push_str(match s {
            "fg:black" | "black" => "\033[30m",
            "fg:red" | "red" => "\u{1b}[31m",
            "fg:green" | "green" => "\u{1b}[32m",
            "fg:orange" | "orange" => "\u{1b}[33m",
            "fg:blue" | "blue" => "\u{1b}[34m",
            "fg:magenta" | "magenta" => "\u{1b}[35m",
            "fg:cyan" | "cyan" => "\u{1b}[36m",
            "fg:gray" | "gray" => "\u{1b}[37m",
            "bg:black" => "\u{1b}[40m",
            "bg:red" => "\u{1b}[41m",
            "bg:green" => "\u{1b}[42m",
            "bg:orange" => "\u{1b}[43m",
            "bg:blue" => "\u{1b}[44m",
            "bg:magenta" => "\u{1b}[45m",
            "bg:cyan" => "\u{1b}[46m",
            "bg:gray" => "\u{1b}[47m",
            "bold" => "\u{1b}[1m",
            "dim" => "\u{1b}[2m",
            "underline" => "\u{1b}[4m",
            _ => "\u{1b}[0m",
        });
    }
    res
}
