use crate::logger;
use crate::Game;
use crate::{
    gid::Gid,
    unit::Stat,
    weapon::Damage,
};

use cherry::graphics::colour::Colour;
use logger::colour;
use rand::Rng;

/*
    Selects a unit. If the unit is invalid, the selected unit
    is unchanged.
*/
pub fn select(game: &mut Game, unit_id: Gid) {
    // Check if the id is valid.
    if game.units.contains(unit_id) {
        // Record the message.
        let unit = &game.units[unit_id];
        game.logger
            .message()
            .with("Selected ", logger::colour::TEXT)
            .with(&unit.name, logger::colour::NAME)
            .with(".", logger::colour::TEXT)
            .build();

        // Update the selected unit.
        game.selected_unit_id = Some(unit_id);

        // Return to the commands menu.
        game.change_menu(game.commands_menu_id, true);
    }
}

/*
    Deselects the selected unit.
*/
pub fn deselect(game: &mut Game) {
    // Check if there is a selected unit.
    if let Some(unit_id) = game.selected_unit_id {
        // Check if the id is valid.
        if game.units.contains(unit_id) {
            // Record the message.
            let unit = &game.units[unit_id];
            game.logger
                .message()
                .with("Deselected ", logger::colour::TEXT)
                .with(&unit.name, logger::colour::NAME)
                .with(".", logger::colour::TEXT)
                .build();
        }

        // Update the selected unit.
        game.selected_unit_id = None;

        // Return to the commands menu.
        game.change_menu(game.commands_menu_id, true);
    }
}

#[derive(Debug, Default)]
struct DamageDealt {
    health: u16,
    armour: u16,
    shield: u16,
    hits: u16,
}

#[derive(Debug, Default)]
struct Vitals {
    health: Stat,
    armour: Stat,
    shield: Stat,
}

/*
    Simulate damage towards a set of vitals. Returns modified vitals,
    damage across each stat, and whether the hit landed.
*/
fn simulate_damage(accuracy: f32, rolls: u16, damage: Damage, vitals: &mut Vitals) -> DamageDealt {
    // Calculate the number of hits.
    let mut rng = rand::thread_rng();
    let mut hits = 0;
    for _roll in 0..rolls {
        if rng.gen::<f32>() <= accuracy {
            hits += 1;
        }
    }

    let mut damage_dealt = DamageDealt::default();

    if hits != 0 {
        // Initialise combat data.
        let mut armour = vitals.armour.val();
        let mut shield = vitals.shield.val();
        let mut damage_to_shield = 0;
        let mut damage_to_armour = 0;
        let mut damage_to_health = 0;
        let mut remaining_damage = damage.base * hits;

        // Shield layer.
        if shield != 0 {
            damage_to_shield += shield.min(damage.energy * hits);
            shield = shield.saturating_sub(damage.energy * hits);
            damage_to_shield += shield.min(remaining_damage);

            let remaining_damage_used = shield.min(remaining_damage);
            shield = shield.saturating_sub(remaining_damage);
            remaining_damage -= remaining_damage_used;
        }

        // Armour layer.
        if shield == 0 && armour != 0 {
            damage_to_armour += armour.min(damage.pierce * hits);
            armour = armour.saturating_sub(damage.pierce * hits);
            damage_to_armour += armour.min(remaining_damage);

            let remaining_damage_used = armour.min(remaining_damage);
            armour = armour.saturating_sub(remaining_damage);
            remaining_damage -= remaining_damage_used;
        }

        // Health layer.
        if shield == 0 && armour == 0 {
            damage_to_health += remaining_damage;
        }

        damage_to_health += damage.explosion * hits;

        // Deal final damage values.
        damage_dealt.health = damage_to_health;
        damage_dealt.armour = damage_to_armour;
        damage_dealt.shield = damage_to_shield;
        damage_dealt.hits = hits;

        vitals.health.change_by(-(damage_to_health as i16));
        vitals.armour.change_by(-(damage_to_armour as i16));
        vitals.shield.change_by(-(damage_to_shield as i16));
    }

    damage_dealt
}

/*
    Cause the selected unit to attack the given target.
    If either is invalid, combat is canceled.
*/
pub fn attack(game: &mut Game, target_id: Gid) {
    // Check if there is a selected unit.
    if let Some(selected_unit_id) = game.selected_unit_id {
        // Check if the id is valid.
        if game.units.contains(selected_unit_id) {
            // Consume stamina.
            let unit = &mut game.units[selected_unit_id];
            unit.stamina.change_by(-(unit.stamina.max() as i16));

            // Get combat stats.
            let unit = &game.units[selected_unit_id];
            let weapon = &game.weapons[unit.weapon_id];
            let damage = weapon.damage;
            let accuracy = unit.accuracy * weapon.accuracy;
            let rolls = weapon.rolls.min(weapon.ammo.val);

            // Consume ammo.
            let weapon = &mut game.weapons[unit.weapon_id];
            weapon.ammo.val = weapon.ammo.val.saturating_sub(rolls);

            // Calculate and deal damage.
            let target = &mut game.units[target_id];
            let mut vitals = Vitals {
                health: target.health,
                armour: target.armour,
                shield: target.shield,
            };

            let damage_dealt = simulate_damage(accuracy, rolls, damage, &mut vitals);

            target.health = vitals.health;
            target.armour = vitals.armour;
            target.shield = vitals.shield;

            // Record the message.
            let unit = &game.units[selected_unit_id];
            let target = &game.units[target_id];
            if damage_dealt.hits != 0 {
                game.logger
                    .message()
                    .with(&unit.name, logger::colour::NAME)
                    .with(" hit (", logger::colour::TEXT)
                    .with(&format!("{}", damage_dealt.hits), logger::colour::NUMBER)
                    .with(") ", logger::colour::TEXT)
                    .with(&target.name, logger::colour::NAME)
                    .with(" for ", logger::colour::TEXT)
                    .with(&format!("{}", damage_dealt.health), logger::colour::HEALTH)
                    .with("|", logger::colour::TEXT)
                    .with(&format!("{}", damage_dealt.armour), logger::colour::ARMOUR)
                    .with("|", logger::colour::TEXT)
                    .with(&format!("{}", damage_dealt.shield), logger::colour::SHIELD)
                    .with(".", logger::colour::TEXT)
                    .build();
            } else {
                game.logger
                    .message()
                    .with(&unit.name, logger::colour::NAME)
                    .with(" failed to hit ", logger::colour::TEXT)
                    .with(&target.name, logger::colour::NAME)
                    .with(".", logger::colour::TEXT)
                    .build();
            }

            // Check if the target is killed.
            if target.health.val() == 0 {
                // Record the message.
                game.logger
                    .message()
                    .with(&target.name, logger::colour::NAME)
                    .with(" was killed!", logger::colour::TEXT)
                    .build();
            }

            // Return to the commands menu.
            game.change_menu(game.commands_menu_id, true);
        }
    } else {
        // The selected unit was invalid, so clear it.
        game.selected_unit_id = None;
    }
}

/*
    Reloads the unit's weapons.
*/
pub fn reload(game: &mut Game, unit_id: Gid) {
    if game.units.contains(unit_id) {
        let unit = game.units.get(unit_id).unwrap();
        let weapon = game.weapons.get_mut(unit.weapon_id).unwrap();
        weapon.ammo.val = weapon.ammo.max;

        let unit = game.units.get_mut(unit_id).unwrap();
        unit.stamina.change_by(-25);

        // Return to the commands menu.
        game.change_menu(game.commands_menu_id, true);
    }
}

/*
    Moves a unit to the destination, if possible, and consumes fifty stamina.
*/
pub fn movement(game: &mut Game, unit_id: Gid, destination_id: Gid) {
    // Check if the unit is valid.
    if let Some(unit) = game.units.get_mut(unit_id) {
        // Check if the area is valid.
        if game.areas.contains(destination_id) {
            // Check if the path is valid.
            let edges = game.world.edges(unit.area_id);

            for (_from, to, _edge) in edges {
                if to == destination_id {
                    unit.area_id = destination_id;
                    unit.stamina.change_by(-50);
                    break;
                }
            }
        }
    } else {
        // The selected unit was invalid, so clear it.
        game.selected_unit_id = None;
    }

    // Return to the commands menu.
    game.change_menu(game.commands_menu_id, true);
}

/*
    Regenerates stamina of the current faction's units according to their speed value.
*/
fn distribute_stamina(game: &mut Game) {
    if let Some(faction_id) = game.current_faction_id {
        for (_unit_id, unit) in &mut game.units {
            unit.stamina.clear_change();

            if unit.faction_id == faction_id {
                unit.stamina.change_by(unit.speed as i16);
            } else {
                unit.health.clear_change();
                unit.armour.clear_change();
                unit.shield.clear_change();
            }
        }
    }
}

/*
    Prunes deceased units and defeated factions from play.
*/
fn prune(game: &mut Game) {
    let mut defeated_factions = Vec::new();

    for (faction_id, faction) in &mut game.factions {
        let mut deceased_units = Vec::new();

        for unit_id in &faction.units {
            let unit = game.units.get(*unit_id).unwrap();
            if unit.health.val() == 0 {
                deceased_units.push(*unit_id);
            }
        }

        // Remove the deceased unit from relevant caches.
        for unit_id in deceased_units {
            let unit = game.units.get(unit_id).unwrap();
            let area = game.areas.get_mut(unit.area_id).unwrap();

            area.units.remove(&unit_id);
            game.units.remove(unit_id);
            faction.units.remove(&unit_id);
        }

        // If the faction has no remaining units, it is defeated.
        if faction.units.is_empty() {
            defeated_factions.push(faction_id);
        }
    }

    // Remove defeated factions from play.
    for faction_id in defeated_factions {
        // Record the message.
        let faction = game.factions.get(faction_id).unwrap();
        game.logger
            .message()
            .with(&faction.name, logger::colour::NAME)
            .with(" was defeated!", logger::colour::TEXT)
            .build();

        // Remove the faction from play.
        game.factions.remove(faction_id);

        // Remove the faction from the turn queue.
        let turn_index = game
            .turn_queue
            .iter()
            .position(|gid| faction_id == *gid)
            .unwrap();

        game.turn_queue.remove(turn_index);
    }
}

/*
    End the turn. The next able faction in the turn queue becomes
    the current faction.
*/
pub fn end_turn(game: &mut Game) {
    // Record the message.
    game.logger
        .message()
        .with("You end the turn.", Colour::DARK_GRAY)
        .build();

    // Prune the deceased and defeated.
    prune(game);

    // Get the next faction in the turn queue.
    let faction_id = game.turn_queue.pop_front();

    if let Some(faction_id) = faction_id {
        // Check if the faction is valid.
        if game.factions.contains(faction_id) {
            // Check if they are the victor.
            if game.turn_queue.is_empty() {
                // Record the message.
                let faction = game.factions.get(faction_id).unwrap();
                game.logger
                    .message()
                    .with(&faction.name, logger::colour::NAME)
                    .with(" is victorious!", colour::TEXT)
                    .build();

                // Clear the current faction.
                game.current_faction_id = None;
            } else {
                // Update the current faction.
                game.current_faction_id = Some(faction_id);

                // Record the message.
                let faction = game.factions.get(faction_id).unwrap();
                let faction_name_with_apostraphe = if faction.name.ends_with('s') {
                    format!("{}'", faction.name)
                } else {
                    format!("{}'s", faction.name)
                };

                game.logger
                    .message()
                    .with(&faction_name_with_apostraphe, logger::colour::NAME)
                    .with(" turn.", logger::colour::TEXT)
                    .build();

                // Push the faction to the rear of the turn queue.
                game.turn_queue.push_back(faction_id);
            }
        } else {
            // The faction was invalid, so clear it.
            game.current_faction_id = None;
        }
    } else {
        // The turn queue was empty. Ensure the current faction
        // is set to none.
        game.current_faction_id = None;
    }

    // Distribute the rations.
    distribute_stamina(game);

    // Reset the selected unit.
    game.selected_unit_id = None;

    // Return to the commands menu.
    game.change_menu(game.commands_menu_id, true);
}

/*
    A placeholder command that records the message 'Nothing happens'.
*/
pub fn empty(game: &mut Game) {
    // Record the message.
    game.logger
        .message()
        .with("Nothing happens.", logger::colour::TEXT)
        .build();
}
