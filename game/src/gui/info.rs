use cherry::{
    engine::Cherry,
    graphics::colour::Colour,
};

use crate::{
    gid::Gid,
    icons,
    unit::Stat,
    Game,
};

pub fn draw_progress_bar(engine: &mut Cherry, x: i32, y: i32, w: i32, percent: f32, dimming: f32) {
    // Cache previous terminal properties.
    let prev_fg = engine.get_fg();
    let prev_bg = engine.get_bg();
    let prev_clip = engine.get_clip();

    // Prepare surface.
    engine.set_fg(Colour::WHITE);
    engine.set_bg(Colour::BLACK);
    engine.fill_rect(x, y, w, 1);

    // Cache data.
    let left_empty = icons::METER_EMPTY_LEFT;
    let middle_empty = icons::METER_EMPTY_MIDDLE;
    let right_empty = icons::METER_EMPTY_RIGHT;
    let left_full = icons::METER_FULL_LEFT;
    let middle_full = icons::METER_FULL_MIDDLE;
    let right_full = icons::METER_FULL_RIGHT;

    // Draw the progress bar.
    engine.set_fg(prev_fg);
    engine.draw_progress_bar_ex(
        x,
        y,
        w,
        percent,
        dimming,
        left_empty,
        middle_empty,
        right_empty,
        left_full,
        middle_full,
        right_full,
    );

    // Restore terminal properties.
    engine.set_fg(prev_fg);
    engine.set_bg(prev_bg);
    engine.set_clip(Some(prev_clip));
}

pub fn draw_stat(engine: &mut Cherry, x: i32, y: i32, w: i32, h: i32, icon: char, stat: &Stat) {
    // Cache previous terminal properties.
    let prev_fg = engine.get_fg();
    let prev_bg = engine.get_bg();
    let prev_clip = engine.get_clip();

    // Prepare surface.
    engine.set_fg(Colour::WHITE);
    engine.set_bg(Colour::BLACK);
    engine.fill_rect(x, y, w, h);

    // Cache data.
    let dimming = 0.65;
    let bar_len = w - 14;
    let percent_full = stat.ratio();

    // Draw Label
    engine.set_fg(Colour::WHITE);
    engine.draw(x, y, icon);
    engine.set_fg(Colour::DARK_GRAY);
    engine.draw(x + 1, y, ':');

    // Draw stat bar.
    engine.set_fg(prev_fg);
    draw_progress_bar(engine, x + 2, y, bar_len, percent_full, dimming);

    // Draw status.
    engine.draw_str(x + bar_len + 3, y, &format!("{}/{}", stat.val, stat.max));
    engine.set_fg(prev_fg * dimming);
    engine.draw_str(x + bar_len + 2, y + 1, &format!("{:+}", stat.change));

    // Restore terminal properties.
    engine.set_fg(prev_fg);
    engine.set_bg(prev_bg);
    engine.set_clip(Some(prev_clip));
}

fn draw_label(engine: &mut Cherry, x: i32, y: i32, icon: char, str: &str) {
    engine.set_fg(Colour::WHITE);
    engine.draw_str(x, y, &format!("{}:{}", icon, str));
    engine.set_fg(Colour::DARK_GRAY);
    engine.draw(x + 1, y, ':');
}

pub fn draw_unit_info(
    game: &Game,
    engine: &mut Cherry,
    unit_id: Gid,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) {
    // Cache previous terminal properties.
    let prev_fg = engine.get_fg();
    let prev_bg = engine.get_bg();
    let prev_clip = engine.get_clip();

    // Prepare surface.
    engine.set_fg(Colour::WHITE);
    engine.set_bg(Colour::BLACK);
    engine.fill_rect(x, y, w, h);

    // Fetch entities.
    let unit = game.units.get(unit_id).unwrap();

    // Unit name.
    engine.set_fg(Colour::DARK_CYAN);
    engine.draw_str(x, y, &unit.name.to_ascii_uppercase());

    // Unit role.
    engine.set_fg(Colour::YELLOW);
    engine.draw_str(x, y + 1, &unit.role);

    // Health bar.
    engine.set_fg(Colour::DARK_RED);
    draw_stat(engine, x, y + 3, 23, 2, icons::HEALTH, &unit.health);

    // Armour bar.
    engine.set_fg(Colour::DARK_GREEN);
    draw_stat(engine, x, y + 5, 23, 2, icons::ARMOUR, &unit.armour);

    // Shield bar.
    engine.set_fg(Colour::DARK_BLUE);
    draw_stat(engine, x, y + 7, 23, 2, icons::SHIELD, &unit.shield);

    // Stamina bar.
    engine.set_fg(Colour::DARK_MAGENTA);
    draw_stat(engine, x, y + 9, 23, 2, icons::ENERGY, &unit.stamina);

    // Restore terminal properties.
    engine.set_fg(prev_fg);
    engine.set_bg(prev_bg);
    engine.set_clip(Some(prev_clip));
}

pub fn draw_weapon_info(
    game: &Game,
    engine: &mut Cherry,
    weapon_id: Gid,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) {
    // Cache previous terminal properties.
    let prev_fg = engine.get_fg();
    let prev_bg = engine.get_bg();
    let prev_clip = engine.get_clip();

    // Prepare surface.
    engine.set_fg(Colour::WHITE);
    engine.set_bg(Colour::BLACK);
    engine.fill_rect(x, y, w, h);

    // Fetch entities.
    let weapon = game.weapons.get(weapon_id).unwrap();

    // Weapon name.
    engine.set_fg(Colour::DARK_CYAN);
    engine.draw_str(x, y, &weapon.name.to_ascii_uppercase());

    // Weapon role.
    engine.set_fg(Colour::YELLOW);
    engine.draw_str(x, y + 1, &weapon.role);

    // Weapon ammo.
    draw_label(
        engine,
        x,
        y + 3,
        icons::CARTRIDGE,
        &format!("{}/{}", weapon.ammo.val, weapon.ammo.max),
    );

    // Weapon accuracy.
    draw_label(
        engine,
        x,
        y + 4,
        icons::TARGET,
        &format!("{:.0}%", weapon.accuracy * 100.0),
    );

    // Weapon rolls.
    draw_label(engine, x, y + 5, icons::DICE, &format!("{}", 3));

    // Weapon base damage.
    draw_label(
        engine,
        x + 11,
        y + 3,
        icons::DAMAGE,
        &format!("{}", weapon.damage.base),
    );

    // Weapon piercing damage.
    draw_label(
        engine,
        x + 11,
        y + 4,
        icons::PIERCING,
        &format!("{}", weapon.damage.pierce),
    );

    // Weapon energy damage.
    draw_label(
        engine,
        x + 11,
        y + 5,
        icons::ENERGY,
        &format!("{}", weapon.damage.energy),
    );

    // Weapon explosive damage.
    draw_label(
        engine,
        x + 11,
        y + 6,
        icons::BOMB,
        &format!("{}", weapon.damage.explosion),
    );

    // Restore terminal properties.
    engine.set_fg(prev_fg);
    engine.set_bg(prev_bg);
    engine.set_clip(Some(prev_clip));
}
