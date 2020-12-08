use cherry::{
    engine::Cherry,
    graphics::colour::Colour,
};

use super::draw_stat;
use crate::unit::Unit;

pub fn draw_unit_info(engine: &mut Cherry, unit: &Unit, x: i32, y: i32, w: i32, h: i32) {
    // Cache terminal colours.
    let prev_fg = engine.get_fg();
    let prev_bg = engine.get_bg();

    // Clip drawing region to space occupied by info panel.
    engine.clip(x, y, w, h, false);

    // Prepare the drawing surface.
    engine.set_fg(Colour::WHITE);
    engine.set_bg(Colour::BLACK);
    engine.fill_rect(x, y, w, h);

    // Draw exterior frame.
    engine.set_fg(Colour::VERY_DARK_CYAN);
    engine.draw_str(x, y, "INFO");
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.draw_border(x, y + 1, w, h);

    // Clip drawing region to the interior of the frame.
    let x = x + 1;
    let y = y + 2;
    let w = w - 2;
    let h = h - 3;

    engine.clip(x, y, w, h, false);

    // Draw unit role.
    engine.set_fg(Colour::YELLOW);
    engine.draw_str(x, y, "Rifleman");

    // Draw health bar.
    engine.set_fg(Colour::DARK_RED);
    draw_stat(engine, x, y + 2, 10, "\u{80}:", unit.health, unit.health_max, 0);

    // Draw armour bar.
    engine.set_fg(Colour::DARK_GREEN);
    draw_stat(engine, x, y + 4, 10, "\u{81}:", unit.armour, unit.armour_max, 0);

    // Draw shield bar.
    engine.set_fg(Colour::DARK_BLUE);
    draw_stat(engine, x, y + 6, 10, "\u{82}:", unit.shield, unit.shield_max, 0);

    // Draw action bar.
    engine.set_fg(Colour::new(160, 80, 0));
    draw_stat(
        engine,
        x,
        y + 8,
        10,
        "\u{94}:",
        unit.actions,
        100,
        unit.speed as i16,
    );

    // Draw weapon info.
    // Draw weapon name.
    engine.set_fg(Colour::YELLOW);
    engine.draw_str(x, y + 10, "M1 Garand");

    // Draw weapon stats.
    engine.set_fg(Colour::WHITE);
    engine.draw_str(x, y + 11, "\u{92}:5/6");
    engine.draw_str(x, y + 12, "\u{95}:70%");
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.draw_str(x + 1, y + 13, "x80%");

    engine.set_fg(Colour::WHITE);
    engine.draw_str(x + 11, y + 11, "\u{93}:35");
    engine.draw_str(x + 11, y + 12, "\u{94}:0");
    engine.draw_str(x + 11, y + 13, "*:0");

    // Undo clipping.
    engine.unclip();

    // Restore terminal colours.
    engine.set_fg(prev_fg);
    engine.set_bg(prev_bg);
}
