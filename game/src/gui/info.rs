use cherry::{
    engine::Cherry,
    graphics::{
        clip::Clip,
        colour::Colour,
    },
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

pub fn draw_stat(
    engine: &mut Cherry,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    icon: char,
    stat: &Stat,
    tip: &str,
) {
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
    engine.draw_str(
        x + bar_len + 3,
        y,
        &format!("{}/{}", stat.val(), stat.max()),
    );
    engine.set_fg(prev_fg * dimming);
    engine.draw_str(x + bar_len + 2, y + 1, &format!("{:+}", stat.change()));

    // Tooltip.
    engine.set_fg(Colour::VERY_DARK_GRAY);
    draw_tooltip(engine, x, y, bar_len + 3, 1, tip);

    // Restore terminal properties.
    engine.set_fg(prev_fg);
    engine.set_bg(prev_bg);
    engine.set_clip(Some(prev_clip));
}

fn draw_label(engine: &mut Cherry, x: i32, y: i32, icon: char, str: &str, tip: &str) {
    // Cache previous terminal properties.
    let prev_fg = engine.get_fg();

    // Label.
    engine.draw_str(x, y, &format!("{}:{}", icon, str));
    engine.set_fg(Colour::DARK_GRAY);
    engine.draw(x + 1, y, ':');

    // Tooltip.
    let label_len = str.chars().count() as i32 + 2;
    engine.set_fg(Colour::VERY_DARK_GRAY);
    draw_tooltip(engine, x, y, label_len, 1, tip);

    // Restore terminal properties.
    engine.set_fg(prev_fg);
}

fn draw_tooltip(engine: &mut Cherry, x: i32, y: i32, w: i32, h: i32, tip: &str) {
    let (mx, my) = engine.mouse_pos();
    let hot_spot = Clip::new(x, y, w, h, false);
    let inside = hot_spot.contains(mx, my);

    if inside {
        let fg = engine.get_fg();
        let tip_len = tip.chars().count() as i32;

        let mut tx = mx;
        let ty = my - 3;
        let spill_over = mx + tip_len - engine.columns() as i32 + 2;
        if spill_over > 0 {
            tx -= spill_over;
        }

        engine.fill_rect(tx, ty, tip_len + 2, 3);
        engine.draw_border(tx, ty, tip_len + 2, 3);
        engine.draw(tx, ty + 2, icons::TIP);
        engine.set_fg(fg * 2.0);
        engine.draw_str(tx + 1, ty + 1, tip);
        engine.set_fg(fg);
    }
}

pub fn draw_unit_info(
    game: &Game,
    engine: &mut Cherry,
    unit_id: Gid,
    selected: bool,
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
    let weapon = game.weapons.get(unit.weapon_id).unwrap();
    let area = game.areas.get(unit.area_id).unwrap();
    let faction = game.factions.get(unit.faction_id).unwrap();

    // Unit name.
    engine.set_fg(Colour::DARK_CYAN);
    engine.draw_str(x, y, &unit.name.to_ascii_uppercase());

    if selected {
        let name_len = unit.name.chars().count() as i32;
        engine.draw(x + name_len + 1, y, '*');
    }

    // Unit role.
    engine.set_fg(Colour::YELLOW);
    engine.draw_str(x, y + 1, &unit.role);

    // Health bar.
    engine.set_fg(Colour::DARK_RED);
    draw_stat(
        engine,
        x,
        y + 3,
        23,
        2,
        icons::HEALTH,
        &unit.health,
        "Health.",
    );

    // Armour bar.
    engine.set_fg(Colour::DARK_GREEN);
    draw_stat(
        engine,
        x,
        y + 5,
        23,
        2,
        icons::ARMOUR,
        &unit.armour,
        "Armour.",
    );

    // Shield bar.
    engine.set_fg(Colour::DARK_BLUE);
    draw_stat(
        engine,
        x,
        y + 7,
        23,
        2,
        icons::SHIELD,
        &unit.shield,
        "Shield.",
    );

    // Stamina bar.
    engine.set_fg(Colour::DARK_MAGENTA);
    draw_stat(
        engine,
        x,
        y + 9,
        23,
        2,
        icons::ENERGY,
        &unit.stamina,
        "Stamina.",
    );

    // Accuracy.
    engine.set_fg(Colour::WHITE);
    draw_label(
        engine,
        x,
        y + 11,
        icons::TARGET,
        &format!("{:.0}%", unit.accuracy * weapon.accuracy * 100.0),
        "Unit accuracy multiplied by weapon accuracy.",
    );
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.draw_str(
        x + 6,
        y + 11,
        &format!("{} {:.0}%", icons::ARROW_LEFT, unit.accuracy * 100.0),
    );
    engine.draw_str(x + 6, y + 12, &format!(" x{:.0}%", weapon.accuracy * 100.0));

    // Area
    engine.set_fg(Colour::WHITE);
    draw_label(engine, x, y + 14, icons::LOCATION, &area.name, "Area.");

    // Faction
    engine.set_fg(Colour::WHITE);
    draw_label(engine, x, y + 15, icons::FLAG, &faction.name, "Faction.");

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
    engine.set_fg(Colour::WHITE);
    draw_label(
        engine,
        x,
        y + 3,
        icons::CARTRIDGE,
        &format!("{}/{}", weapon.ammo.val, weapon.ammo.max),
        "Ammunition.",
    );

    // Weapon accuracy.
    draw_label(
        engine,
        x,
        y + 4,
        icons::TARGET,
        &format!("{:.0}%", weapon.accuracy * 100.0),
        "Accuracy.",
    );

    // Weapon rolls.
    draw_label(
        engine,
        x,
        y + 5,
        icons::DICE,
        &format!("{}", weapon.rolls),
        "Rolls.",
    );

    // Weapon weight.
    draw_label(
        engine,
        x,
        y + 6,
        icons::WEIGHT,
        &format!("{}", weapon.weight),
        "Weight.",
    );

    // Weapon base damage.
    draw_label(
        engine,
        x + 13,
        y + 3,
        icons::DAMAGE,
        &format!("{}", weapon.damage.base),
        "Base damage.",
    );

    // Weapon piercing damage.
    draw_label(
        engine,
        x + 13,
        y + 4,
        icons::PIERCING,
        &format!("{}", weapon.damage.pierce),
        "Piercing damage, bonus against armour.",
    );

    // Weapon energy damage.
    draw_label(
        engine,
        x + 13,
        y + 5,
        icons::ENERGY,
        &format!("{}", weapon.damage.energy),
        "Energy damage, bonus against shield.",
    );

    // Weapon explosive damage.
    draw_label(
        engine,
        x + 13,
        y + 6,
        icons::BOMB,
        &format!("{}", weapon.damage.explosion),
        "Explosive damage, ignores armour and shield.",
    );

    // Restore terminal properties.
    engine.set_fg(prev_fg);
    engine.set_bg(prev_bg);
    engine.set_clip(Some(prev_clip));
}

pub fn draw_area_info(
    game: &Game,
    engine: &mut Cherry,
    area_id: Gid,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) {
    // Cache previous terminal properties.
    let prev_fg = engine.get_fg();
    let prev_bg = engine.get_bg();

    // Prepare surface.
    engine.set_fg(Colour::WHITE);
    engine.set_bg(Colour::BLACK);
    engine.fill_rect(x, y, w, h);

    // Fetch entities.
    let area = game.areas.get(area_id).unwrap();

    // Area name.
    engine.set_fg(Colour::DARK_CYAN);
    engine.draw_str(x, y, &area.name.to_ascii_uppercase());

    // Inhabitants.
    if !area.units.is_empty() {
        let mut i = 0;
        for (faction_id, faction) in &game.factions {
            let units: Vec<Gid> = game
                .units
                .iter()
                .filter_map(|(unit_id, unit)| {
                    if unit.area_id == area_id
                        && unit.faction_id == faction_id
                        && unit.health.val() != 0
                    {
                        Some(unit_id)
                    } else {
                        None
                    }
                })
                .collect();

            if units.is_empty() {
                continue;
            }

            i += 1;

            engine.set_fg(Colour::YELLOW);
            engine.draw_str(x, y + i, &faction.name);
            engine.set_fg(Colour::WHITE);

            i += 1;

            for unit_id in units {
                let unit = game.units.get(unit_id).unwrap();
                engine.draw_str(x + 1, y + i as i32, &unit.name);
                i += 1;
            }
        }
    } else {
        engine.set_fg(Colour::DARK_GRAY);
        engine.draw_str(x + 1, y + 1, "Not a whisper nor soul.");
    }

    // Restore terminal properties.
    engine.set_fg(prev_fg);
    engine.set_bg(prev_bg);
}
