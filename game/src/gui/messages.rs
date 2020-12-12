use crate::Message;

use std::collections::VecDeque;

use cherry::{
    engine::Cherry,
    graphics::colour::Colour,
};

pub fn draw_messages(
    engine: &mut Cherry,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    messages: &VecDeque<Message>,
) {
    // Setup
    engine.fill_rect(x, y, w, h);

    // Title
    engine.set_fg(Colour::VERY_DARK_CYAN);
    engine.draw_str(x, y, "MESSAGES");

    // Separator
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.draw_h_line(x, y + 1, w, 0xC4 as char);

    // Messages
    let max_messages = h - 3;

    for (i, message) in messages.iter().enumerate() {
        if i as i32 >= max_messages {
            break;
        }

        let colour = if i == 0 {
            message.colour
        } else {
            message.colour * 0.5
        };

        engine.set_fg(colour);
        engine.draw_str(
            x,
            y + 2 + i as i32,
            &message.content[..(w as usize).min(message.content.len())],
        );
    }

    // Separator
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.draw_h_line(x, y + h - 2, w, 0xC4 as char);
}
