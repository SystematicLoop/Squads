use cherry::{
    engine::Cherry,
    graphics::colour::Colour,
};

pub mod menu;
pub mod messages;

pub fn draw_stat(
    engine: &mut Cherry,
    x: i32,
    y: i32,
    w: i32,
    label: &str,
    val: u16,
    max: u16,
    change: i16,
) {
    let fg = engine.get_fg();
    let label_len = label.chars().count() as i32;
    let percent = val as f32 / max as f32;

    engine.set_fg(Colour::WHITE);
    engine.draw_str(x, y, label);
    engine.set_fg(fg);
    engine.draw_progress_bar_ex(x + label_len, y, w, percent, 0.5);
    engine.draw_str(x + label_len + w + 1, y, &format!("{}/{}", val, max));
    engine.set_fg(fg * 0.5);
    engine.draw_str(x + 12, y + 1, &format!("{:+}", change));
}
