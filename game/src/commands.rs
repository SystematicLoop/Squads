use cherry::graphics::colour::Colour;

use crate::{
    Game,
    Gid,
};

/*
    Selects a unit. If the unit is invalid, the selected unit
    is unchanged.
*/
pub fn select(game: &mut Game, unit_id: Gid) {
    // Check if the id is valid.
    if game.units.contains(unit_id) {
        // Record the message.
        let unit = &game.units[unit_id];
        let message = format!("Selected {}.", unit.name);
        game.record_message(&message, Colour::DARK_GRAY);

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
            let message = format!("Deselected {}.", unit.name);
            game.record_message(&message, Colour::DARK_GRAY);
        }

        // Update the selected unit.
        game.selected_unit_id = None;

        // Return to the commands menu.
        game.change_menu(game.commands_menu_id, true);
    }
}

/*
    Cause the selected unit to attack the given target.
    If either is invalid, combat is canceled.
*/
pub fn attack(game: &mut Game, target_id: Gid) {
    // Check if there is a selected unit.
    if let Some(id) = game.selected_unit_id {
        // Check if the id is valid.
        if game.units.contains(id) {
            // Record the message.
            let unit = &game.units[game.selected_unit_id.unwrap()];
            let target = &game.units[target_id];
            let message = format!("{} attacked {}.", unit.name, target.name);
            game.record_message(&message, Colour::DARK_GRAY);

            // Calculate and deal damage.
            let mut target = &mut game.units[target_id];
            target.health.val = 0;

            // Check if the target is killed.
            if target.health.val == 0 {
                // Record the message.
                let message = format!("{} was killed.", target.name);
                game.record_message(&message, Colour::RED);
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
        let message = format!("{} was defeated.", faction.name);
        game.record_message(&message, Colour::RED);

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
    game.record_message("You end the turn.", Colour::DARK_GRAY);

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
    game.record_message("Nothing happens.", Colour::DARK_GRAY);
}
