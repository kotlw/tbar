mod component;
mod config;
mod options;

use zellij_tile::prelude::*;

use colored::Color;

use crate::component::style::Style;
use crate::component::text::Text;
use crate::component::traits::Component;
use crate::options::Options;
use color_print::cformat;
use handlebars::Handlebars;
use serde_json::json;

#[derive(Default)]
struct FooBar {
    mode_info: ModeInfo,
    options: Options,
    // components: Vec<Box<dyn Component>>,
}

register_plugin!(FooBar);

impl ZellijPlugin for FooBar {
    fn load(&mut self) {
        set_selectable(false);
        subscribe(&[EventType::ModeUpdate]);
        // self.components = vec![Text::new(
        //     "Hello world".to_string(),
        //     Style::new((1, 1), false, Some(Color::BrightGreen), None),
        // )]
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    should_render = true;
                }
                self.mode_info = mode_info
            }
            _ => {}
        }
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        // let mut s = String::new();
        // for cmp in self.components.iter() {
        //     s = s + &cmp.render();
        // }
        // print!("{}", s);
        let reg = Handlebars::new();
        let mut res = self.options.line.clone();

        if let Ok(r) =
            reg.render_template(&res, &json!({"S": self.mode_info.session_name.as_deref(),
            "green": "\u{1b}[32m", "default": "\u{1b}[39m"}))
        {
            res = r
        }

        print!("{}", res);
    }
}
