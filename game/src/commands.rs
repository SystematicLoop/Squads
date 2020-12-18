use cherry::graphics::colour::Colour;

use crate::gid::Gid;
use crate::logger;
use crate::Game;

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

struct FinalDamage {
    health: u16,
    armour: u16,
    shield: u16,
}

/*
    Description
*/
fn get_final_damage(game: &Game, unit_id: Gid, target_id: Gid) -> FinalDamage {
    // Retrieve relevant entities.
    let unit = game.units.get(unit_id).unwrap();
    let target = game.units.get(target_id).unwrap();
    let weapon = game.weapons.get(unit.weapon_id).unwrap();

    // Initialise combat data.
    let mut damage_to_health = 0;
    let mut damage_to_armour = 0;
    let mut damage_to_shield = 0;
    let mut remaining_damage = weapon.damage.base;
    let mut penetrate_shield = false;

    // Shield layer:
    // The energy shield will absorb incoming damage. Its integrity
    // lost is equal to the incoming damage. Any damage greater than
    // its current integrity carries over into the armour layer.
    if target.shield.val != 0 {
        damage_to_shield = target.shield.val.min(remaining_damage);
        remaining_damage -= damage_to_shield;

        // Check if shield is compromised.
        if remaining_damage != 0 {
            penetrate_shield = true;
        }
    } else {
        penetrate_shield = true;
    }

    // Armour layer:
    // The armour will reduce incoming damage equal to the ratio
    // of the maximum armour value to the current armour value.
    // The durability will also be reduced by the raw incoming damage.
    if penetrate_shield && target.armour.val != 0 {
        let armour_coefficient = 1.0 - target.armour.ratio();
        damage_to_armour = target.armour.val.min(remaining_damage);
        remaining_damage = (remaining_damage as f32 * armour_coefficient) as u16;
    }

    // Health layer.
    if penetrate_shield {
        damage_to_health = target.health.val.min(remaining_damage);
    }

    FinalDamage {
        health: damage_to_health,
        armour: damage_to_armour,
        shield: damage_to_shield,
    }
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
            // Calculate and deal damage.
            let damage = get_final_damage(game, selected_unit_id, target_id);
            let mut target = game.units.get_mut(target_id).unwrap();
            target.health.val -= damage.health;
            target.armour.val -= damage.armour;
            target.shield.val -= damage.shield;

            // Record the message.
            let unit = &game.units[selected_unit_id];
            let target = &game.units[target_id];
            game.logger
                .message()
                .with(&unit.name, logger::colour::NAME)
                .with(" hit ", logger::colour::TEXT)
                .with(&target.name, logger::colour::NAME)
                .with(" for ", logger::colour::TEXT)
                .with(&format!("{}", damage.health), logger::colour::HEALTH)
                .with("|", logger::colour::TEXT)
                .with(&format!("{}", damage.armour), logger::colour::ARMOUR)
                .with("|", logger::colour::TEXT)
                .with(&format!("{}", damage.shield), logger::colour::SHIELD)
                .with(".", logger::colour::TEXT)
                .build();

            // Check if the target is killed.
            if target.health.val == 0 {
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
    Prunes deceased units and defeated factions from play.
*/
fn prune(game: &mut Game) {
    let mut defeated_factions = Vec::new();

    for (faction_id, faction) in &mut game.factions {
        let mut deceased_units = Vec::new();

        for unit_id in &faction.units {
            let unit = game.units.get(*unit_id).unwrap();
            if unit.health.val == 0 {
                deceased_units.push(*unit_id);
            }
        }

        // Remove the deceased unit from relevant caches.
        for unit_id in deceased_units {
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
            // Update the current faction.
            game.current_faction_id = Some(faction_id);

            // Push the faction to the rear of the turn queue.
            game.turn_queue.push_back(faction_id);
        } else {
            // The faction was invalid, so clear it.
            game.current_faction_id = None;
        }
    } else {
        // The turn queue was empty. Ensure the current faction
        // is set to none.
        game.current_faction_id = None;
    }

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
