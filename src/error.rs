use crate::style;
use std::cmp;
use zellij_tile::prelude::*;

pub fn render(
    palette: &Palette,
    cols: usize,
    hint: &String,
    layout: &String,
    hl_begin: usize,
    hl_end: usize,
) -> String {
    let bg_color = style::render(palette, &style::Style::Bg(style::Color::Red));
    let fg_color = style::render(palette, &style::Style::Fg(style::Color::Black));
    let hl_color = style::render(palette, &style::Style::Bg(style::Color::Yellow));

    let layout_len = layout.chars().count();
    let hint_len = hint.chars().count();
    let hl_len = hl_end.saturating_sub(hl_begin);
    let layout_bounds = cols.saturating_sub(hint_len);

    // Calculate layout window beginning and end
    let offset = layout_bounds.saturating_sub(hl_len + 6) / 2;
    let layout_begin = hl_begin.saturating_sub(offset);
    let layout_end = cmp::min(layout_len, hl_end + offset);

    // Setup layout wrapping strings
    let before_layout = if layout_begin > 0 { "..." } else { "^" };
    let after_layout = if layout_end < layout_len { "..." } else { "$" };

    // Calculate spacer len. It to fill bar with color till the end of the string.
    let visible_layout_len = layout_end.saturating_sub(layout_begin);
    let layout_wrapps_len = before_layout.chars().count() + after_layout.chars().count();
    let spacer_len = cols.saturating_sub(hint_len + visible_layout_len + layout_wrapps_len);
    let spacer = " ".repeat(spacer_len);

    // Squeeze highlighted text if needed.
    let squeeze_size = (hint_len + hl_len + 6).saturating_sub(cols);
    let hl_end_squeezed = cmp::max(hl_begin, hl_end.saturating_sub(squeeze_size));
    let mut highlight = hl_color.to_string() + &layout[hl_begin..hl_end_squeezed] + &bg_color;

    // Calculate offset (len of non displayable chars).
    let mut offset = bg_color.chars().count() + hl_color.chars().count();
    if hl_end_squeezed <= hl_begin {
        offset = 0;
        highlight = "".to_string();
    };

    let before_hl = before_layout.to_string() + &layout[layout_begin..hl_begin];
    let after_hl = layout[hl_end..layout_end].to_string() + &after_layout;
    let msg = hint.to_string() + &before_hl + &highlight + &after_hl + &spacer;

    bg_color + &fg_color + &msg.chars().take(cols + offset).collect::<String>()
}

// #[derive(Default)]
// pub struct ErrorRenderer {
//     style_renderer: StyleRenderer,
// }
//
// impl ErrorRenderer {
//     pub fn update(&mut self, palette: Palette) -> bool {
//         self.style_renderer.update(palette)
//     }
//
//     pub fn render(
//         &self,
//         cols: usize,
//         hint: &String,
//         layout: &String,
//         hl_begin: usize,
//         hl_end: usize,
//     ) -> String {
//         let bg_color = self.style_renderer.render(&Style::Bg(Color::Red));
//         let fg_color = self.style_renderer.render(&Style::Fg(Color::Black));
//         let hl_color = self.style_renderer.render(&Style::Bg(Color::Yellow));
//
//         let layout_len = layout.chars().count();
//         let hint_len = hint.chars().count();
//         let hl_len = hl_end.saturating_sub(hl_begin);
//         let layout_bounds = cols.saturating_sub(hint_len);
//
//         // Calculate layout window beginning and end
//         let offset = layout_bounds.saturating_sub(hl_len + 6) / 2;
//         let layout_begin = hl_begin.saturating_sub(offset);
//         let layout_end = cmp::min(layout_len, hl_end + offset);
//
//         // Setup layout wrapping strings
//         let before_layout = if layout_begin > 0 { "..." } else { "^" };
//         let after_layout = if layout_end < layout_len { "..." } else { "$" };
//
//         // Calculate spacer len. It to fill bar with color till the end of the string.
//         let visible_layout_len = layout_end.saturating_sub(layout_begin);
//         let layout_wrapps_len = before_layout.chars().count() + after_layout.chars().count();
//         let spacer_len = cols.saturating_sub(hint_len + visible_layout_len + layout_wrapps_len);
//         let spacer = " ".repeat(spacer_len);
//
//         // Squeeze highlighted text if needed.
//         let squeeze_size = (hint_len + hl_len + 6).saturating_sub(cols);
//         let hl_end_squeezed = cmp::max(hl_begin, hl_end.saturating_sub(squeeze_size));
//         let mut highlight = hl_color.to_string() + &layout[hl_begin..hl_end_squeezed] + &bg_color;
//
//         // Calculate offset (len of non displayable chars).
//         let mut offset = bg_color.chars().count() + hl_color.chars().count();
//         if hl_end_squeezed <= hl_begin {
//             offset = 0;
//             highlight = "".to_string();
//         };
//
//         let before_hl = before_layout.to_string() + &layout[layout_begin..hl_begin];
//         let after_hl = layout[hl_end..layout_end].to_string() + &after_layout;
//         let msg = hint.to_string() + &before_hl + &highlight + &after_hl + &spacer;
//
//         bg_color + &fg_color + &msg.chars().take(cols + offset).collect::<String>()
//     }
// }
