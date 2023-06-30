use ansi_term::{Color, Style};
use zellij_tile::prelude::*;

fn as_color(color: PaletteColor) -> Color {
    match color {
        PaletteColor::Rgb((r, g, b)) => Color::RGB(r, g, b),
        PaletteColor::EightBit(color) => Color::Fixed(color),
    }
}

pub fn apply(style: &String, palette: Palette) -> String {
    let mut res = String::new();
    let mut ansi_style = Style::new();

    if style == "default" {
        return ansi_style.on(Color::Fixed(0)).suffix().to_string();
    }

    for s in style.split(',') {
        ansi_style = match s {
            "bold" => ansi_style.bold(),
            "fg:black" => ansi_style.fg(as_color(palette.black)),
            "fg:red" => ansi_style.fg(as_color(palette.red)),
            "fg:green" => ansi_style.fg(as_color(palette.green)),
            "fg:yellow" => ansi_style.fg(as_color(palette.yellow)),
            "fg:blue" => ansi_style.fg(as_color(palette.blue)),
            "fg:magenta" => ansi_style.fg(as_color(palette.magenta)),
            "fg:cyan" => ansi_style.fg(as_color(palette.cyan)),
            "fg:white" => ansi_style.fg(as_color(palette.white)),
            "fg:orange" => ansi_style.fg(as_color(palette.orange)),
            "fg:gray" => ansi_style.fg(as_color(palette.gray)),
            "fg:purple" => ansi_style.fg(as_color(palette.purple)),
            "fg:gold" => ansi_style.fg(as_color(palette.gold)),
            "fg:silver" => ansi_style.fg(as_color(palette.silver)),
            "fg:pink" => ansi_style.fg(as_color(palette.pink)),
            "fg:brown" => ansi_style.fg(as_color(palette.brown)),

            "bg:black" => ansi_style.on(as_color(palette.black)),
            "bg:red" => ansi_style.on(as_color(palette.red)),
            "bg:green" => ansi_style.on(as_color(palette.green)),
            "bg:yellow" => ansi_style.on(as_color(palette.yellow)),
            "bg:blue" => ansi_style.on(as_color(palette.blue)),
            "bg:magenta" => ansi_style.on(as_color(palette.magenta)),
            "bg:cyan" => ansi_style.on(as_color(palette.cyan)),
            "bg:white" => ansi_style.on(as_color(palette.white)),
            "bg:orange" => ansi_style.on(as_color(palette.orange)),
            "bg:gray" => ansi_style.on(as_color(palette.gray)),
            "bg:purple" => ansi_style.on(as_color(palette.purple)),
            "bg:gold" => ansi_style.on(as_color(palette.gold)),
            "bg:silver" => ansi_style.on(as_color(palette.silver)),
            "bg:pink" => ansi_style.on(as_color(palette.pink)),
            "bg:brown" => ansi_style.on(as_color(palette.brown)),
            _ => ansi_style,
        };

        res.push_str(&ansi_style.prefix().to_string());
    }
    res
}
