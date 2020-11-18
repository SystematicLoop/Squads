use crate::Game;
use cherry::{engine::Engine, input::key::Key};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CommandsMenu {
    Select,
    Attack,
    Move,
    EndTurn,
}

#[derive(Debug, Copy, Clone)]
pub struct Result {
    pub select: CommandsMenu,
    pub chosen: bool,
}

pub fn commands_menu(game: &Game, engine: &mut Engine, select: CommandsMenu) -> Result {
    // ------------------------------------
    // Determine valid menu items.
    let mut item_names = Vec::new();
    let mut item_enums = Vec::new();

    // Select
    if game.selectable.len() != 0 {
        item_names.push("Select");
        item_enums.push(CommandsMenu::Select);
    }

    // Attack...
    item_names.push("Attack");
    item_enums.push(CommandsMenu::Attack);

    // Move...
    item_names.push("Move");
    item_enums.push(CommandsMenu::Move);

    // End Turn
    item_names.push("End Turn");
    item_enums.push(CommandsMenu::EndTurn);

    // The commands menu should never be empty.
    assert_eq!(item_names.len(), item_enums.len());
    assert!(item_names.len() != 0);

    let mut select_index = item_enums.iter().position(|x| *x == select).unwrap();
    
    // ------------------------------------
    // Draw menu.
    super::menu::draw_menu(engine, 1, 1, "COMMANDS", &item_names, select_index);

    // ------------------------------------
    // Handle input.
    if engine.key(Key::Enter).just_down {
        Result {
            select: item_enums[select_index],
            chosen: true,
        }
    } else if engine.key(Key::Up).just_down {
        select_index = select_index.saturating_sub(1);

        Result {
            select: item_enums[select_index],
            chosen: false,
        }
    } else if engine.key(Key::Down).just_down {
        select_index = (select_index + 1).min(item_names.len() - 1);

        Result {
            select: item_enums[select_index],
            chosen: false,
        }
    } else {
        // No response.
        
        Result {
            select: item_enums[select_index],
            chosen: false,
        }
    }
}