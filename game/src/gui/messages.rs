use crate::logger::Message;

use std::collections::VecDeque;

use cherry::{
    engine::Cherry,
    graphics::{
        clip::Clip,
        colour::Colour,
    },
};

#[derive(Clone)]
struct Fragment<'a> {
    x: i32,
    y: i32,
    content: &'a str,
    colour: Colour,
}

/*
    Organises the messages into discrete lines of tokens for ease of printing.
*/
fn organise_messages<'a>(w: i32, h: i32, messages: &'a VecDeque<Message>) -> Vec<Fragment<'a>> {
    let mut fragments = Vec::new();

    // The position of the cursor.
    let mut cx = 0;
    let mut cy = 0;

    'outer: for (_id, message) in messages.iter().rev().enumerate() {
        for token in &message.tokens {
            for fragment in token.content.split_inclusive(' ').collect::<Vec<&str>>() {
                let len = fragment.chars().count() as i32;

                // Check if the next fragment will overflow the line.
                if cx + len > w {
                    // Wrap the cursor to the beginning of the next line.
                    cx = 0;
                    cy += 1;

                    if len > w {
                        // We ignore this token and continue with the next token.
                        break;
                    }
                }

                if cy >= h {
                    // We ran out of room vertically.
                    break 'outer;
                }

                // Add the fragment to the line.
                let fragment = Fragment {
                    x: cx,
                    y: cy,
                    content: fragment,
                    colour: token.colour,
                };

                fragments.push(fragment);

                // Advance the cursor by the length of the fragment.
                cx += len;
            }
        }

        // Wrap the cursor around when we reach the next message.
        cx = 0;
        cy += 1;
    }

    fragments
}

pub fn draw_messages(
    engine: &mut Cherry,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    messages: &VecDeque<Message>,
) {
    // Cache previous terminal properties.
    let prev_fg = engine.get_fg();
    let prev_bg = engine.get_bg();
    let prev_clip = engine.get_clip();

    // Clip drawing region to space occupied by info panel.
    engine.set_clip(Some(Clip::new(x, y, w, h, false)));

    // Setup.
    engine.fill_rect(x, y, w, h);

    // Title.
    engine.set_fg(Colour::DARK_CYAN);
    engine.draw_str(x, y, "MESSAGES");

    // Opening separator.
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.draw_h_line(x, y + 1, w, 0xC4 as char);

    // Closing separator.
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.draw_h_line(x, y + h - 2, w, 0xC4 as char);

    // Messages.
    let max_messages = h - 4;
    let fragments = organise_messages(w, max_messages, messages);
    for fragment in fragments {
        engine.set_fg(fragment.colour);
        engine.draw_str(x + fragment.x, y + fragment.y + 2, &fragment.content);
    }

    // Restore previous terminal properties.
    engine.set_fg(prev_fg);
    engine.set_bg(prev_bg);
    engine.set_clip(Some(prev_clip));
}
