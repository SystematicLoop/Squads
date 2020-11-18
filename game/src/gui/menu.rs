use cherry::{
    engine::Engine,
    graphics::colour::Colour,
};

pub fn draw_menu(
    engine: &mut Engine,
    x: i32,
    y: i32,
    title: &str,
    items: &[&str],
    selection: usize,
) {
    // Title
    engine.set_fg(Colour::VERY_DARK_CYAN);
    engine.draw_str(x, y, title);

    // Separator
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.draw_h_line(x, y + 1, 16, 0xC4 as char);

    // Menu items
    for (i, item) in items.iter().enumerate() {
        if i == selection {
            engine.set_fg(Colour::BLACK);
            engine.set_bg(Colour::WHITE);
            engine.draw_str(x, y + 2 + i as i32, &format!("> {:14}", item));
        } else {
            engine.set_fg(Colour::GRAY);
            engine.set_bg(Colour::BLACK);
            engine.draw_str(x, y + 2 + i as i32, &format!("  {:14}", item));
        }
    }

    // Separator
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.set_bg(Colour::BLACK);
    engine.draw_h_line(x, y + items.len() as i32 + 2, 16, 0xC4 as char);
}
