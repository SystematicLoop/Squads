pub mod commands;
pub mod faction;
pub mod gid;
pub mod gui;
pub mod scenario;
pub mod serde;
pub mod unit;

use std::collections::{
    HashSet,
    VecDeque,
};

use cherry::{
    engine::Cherry,
    graphics::colour::Colour,
    input::key::Key,
    CherryApp,
};

use faction::{
    Faction,
    FactionDef,
};

use gid::{
    Arena,
    Gid,
};

use gui::{
    info::draw_unit_info,
    menu::{
        draw_menu,
        Menu,
    },
    messages::draw_messages,
};

use scenario::Scenario;

use unit::{
    Unit,
    UnitDef,
    UnitSpawn,
};

#[derive(Default)]
pub struct Game {
    // World
    pub factions: Arena<Faction>,
    pub units: Arena<Unit>,

    // Data caches
    pub current_faction_id: Option<Gid>,
    pub turn_queue: VecDeque<Gid>,

    pub selected_unit_id: Option<Gid>,
    pub target_unit_id: Option<Gid>,

    pub selectable: Vec<Gid>,
    pub targetable: Vec<Gid>,

    pub messages: VecDeque<Message>,

    // Menus
    pub menus: Vec<Menu<MenuData>>,
    pub menu_id: usize,
    pub item_id: usize,
    pub menu_changed: bool,
    pub commands_menu_id: usize,
    pub select_menu_id: usize,
    pub attack_menu_id: usize,
}

impl Game {
    pub fn change_menu(&mut self, menu_id: usize, force_menu_changed: bool) {
        if self.menu_id != menu_id || force_menu_changed {
            self.menu_id = menu_id;
            self.item_id = 0;
            self.menu_changed = true;
        }
    }

    pub fn record_message(&mut self, content: &str, colour: Colour) {
        let message = Message {
            content: String::from(content),
            colour,
        };

        self.messages.push_front(message);

        // Todo: Remove this magic number.
        while self.messages.len() >= 10 {
            self.messages.pop_back();
        }
    }

    pub fn update_commands_menu(&mut self) {
        // Reset data caches.
        self.selectable.clear();
        self.targetable.clear();

        if let Some(faction_id) = self.current_faction_id {
            // We assume the current faction is valid.
            let faction = self.factions.get(faction_id).unwrap();

            // Update selectable units.
            for unit_id in &faction.units {
                if let Some(selected_unit_id) = self.selected_unit_id {
                    if *unit_id == selected_unit_id {
                        // Skip the already selected unit.
                        continue;
                    }
                }

                let unit = &self.units[*unit_id];

                if unit.health.val != 0 {
                    self.selectable.push(*unit_id);
                }
            }

            // Update targetable units.
            if let Some(selected_unit_id) = self.selected_unit_id {
                let unit = &self.units[selected_unit_id];

                for (target_id, target) in &self.units {
                    if target.faction != unit.faction && target.health.val != 0 {
                        self.targetable.push(target_id);
                    }
                }
            }
        }

        // Update menu items.
        let menu = &mut self.menus[self.commands_menu_id];
        menu.clear();

        if self.selectable.len() != 0 {
            menu.add(
                "Select",
                MenuData::ChangeMenu {
                    id: self.select_menu_id,
                },
            );
        }

        if self.selected_unit_id.is_some() {
            menu.add("Deselect", MenuData::DeselectUnit);
        }

        if self.targetable.len() != 0 {
            menu.add(
                "Attack",
                MenuData::ChangeMenu {
                    id: self.attack_menu_id,
                },
            );
        }

        menu.add("End Turn", MenuData::EndTurn);
    }

    pub fn update_select_menu(&mut self) {
        // Update menu items.
        let menu = &mut self.menus[self.select_menu_id];
        menu.clear();

        for id in &self.selectable {
            let unit = &self.units[*id];
            menu.add(&unit.name, MenuData::SelectUnit { id: *id });
        }

        menu.add(
            "Back",
            MenuData::ChangeMenu {
                id: self.commands_menu_id,
            },
        );
    }

    pub fn update_attack_menu(&mut self) {
        // Update menu items.
        let menu = &mut self.menus[self.attack_menu_id];
        menu.clear();

        for id in &self.targetable {
            let unit = &self.units[*id];
            menu.add(&unit.name, MenuData::AttackUnit { id: *id });
        }

        menu.add(
            "Back",
            MenuData::ChangeMenu {
                id: self.commands_menu_id,
            },
        )
    }
}

impl CherryApp for Game {
    fn on_update(&mut self, engine: &mut Cherry) {
        // Update menu.
        if self.menu_changed {
            self.menu_changed = false;

            if self.menu_id == self.commands_menu_id {
                self.update_commands_menu();
            }

            if self.menu_id == self.select_menu_id {
                self.update_select_menu();
            }

            if self.menu_id == self.attack_menu_id {
                self.update_attack_menu();
            }
        }

        // Clear view.
        engine.set_fg(Colour::WHITE);
        engine.set_bg(Colour::BLACK);
        engine.clear();

        // Draw border.
        engine.set_fg(Colour::new(16, 16, 16));
        engine.draw_border(0, 0, 60, 40);

        // Draw menu.
        let menu = &self.menus[self.menu_id];
        draw_menu(engine, 1, 1, 25, 13, &menu, self.item_id);

        if self.menu_id == self.select_menu_id {
            let menu = &self.menus[self.menu_id];
            let item = menu.get(self.item_id).unwrap();

            match item.data() {
                MenuData::SelectUnit { id } => {
                    let unit = &self.units[*id];
                    draw_unit_info(engine, &unit, 1, 14, 26, 16);
                }
                _ => {}
            }
        } else if self.menu_id == self.attack_menu_id {
            let menu = &self.menus[self.menu_id];
            let item = menu.get(self.item_id).unwrap();

            match item.data() {
                MenuData::AttackUnit { id } => {
                    let unit = &self.units[*id];
                    draw_unit_info(engine, &unit, 1, 14, 26, 16);
                }
                _ => {}
            }
        }

        // Draw messages.
        draw_messages(engine, 27, 1, 32, 13, &self.messages);

        // Input.
        if engine.key(Key::Up).just_down {
            if self.item_id == 0 {
                self.item_id = menu.len().saturating_sub(1);
            } else {
                self.item_id -= 1;
            }
        }

        if engine.key(Key::Down).just_down {
            if self.item_id == menu.len().saturating_sub(1) {
                self.item_id = 0;
            } else {
                self.item_id += 1;
            }
        }

        if engine.key(Key::Enter).just_down {
            if let Some(item) = menu.get(self.item_id) {
                let data = item.data().clone();
                match data {
                    MenuData::ChangeMenu { id } => {
                        self.change_menu(id, true);
                    }
                    MenuData::SelectUnit { id } => {
                        commands::select(self, id);
                    }
                    MenuData::DeselectUnit => {
                        commands::deselect(self);
                    }
                    MenuData::AttackUnit { id } => {
                        commands::attack(self, id);
                    }
                    MenuData::EndTurn => {
                        commands::end_turn(self);
                    }
                    MenuData::Empty => {
                        commands::empty(self);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub content: String,
    pub colour: Colour,
}

#[derive(Debug, Copy, Clone)]
pub enum MenuData {
    ChangeMenu { id: usize },
    SelectUnit { id: Gid },
    AttackUnit { id: Gid },
    DeselectUnit,
    EndTurn,
    Empty,
}

fn main() {
    let mut game = Game::default();

    // Initialise menus.
    {
        game.commands_menu_id = game.menus.len();
        game.menus.push(Menu::new("COMMANDS"));

        game.select_menu_id = game.menus.len();
        game.menus.push(Menu::new("SELECT"));

        game.attack_menu_id = game.menus.len();
        game.menus.push(Menu::new("ATTACK"));

        game.change_menu(game.commands_menu_id, true);
    }

    // Load scenario.
    let scenario = serde::deserialise_file::<Scenario>("res/scenarios/dev/scenario.json");

    // Load factions.
    {
        let faction_defs = serde::deserialise_dir::<FactionDef>("res/scenarios/dev/factions/");
        for faction_def in faction_defs {
            let faction = Faction {
                name: faction_def.name,
                units: HashSet::new(),
            };

            let is_starting_faction = faction.name == scenario.starting_faction;
            let faction_id = game.factions.insert(faction);

            game.turn_queue.push_back(faction_id);

            if is_starting_faction {
                game.current_faction_id = Some(faction_id);
            }
        }

        assert!(game.current_faction_id.is_some());
    }

    // Load units.
    {
        let unit_defs = serde::deserialise_dir::<UnitDef>("res/scenarios/dev/units/");
        let spawns = serde::deserialise_file::<Vec<UnitSpawn>>("res/scenarios/dev/spawns.json");

        for spawn in spawns {
            // Retrieve the faction by name.
            let faction_id = game
                .factions
                .iter()
                .find(|(_id, faction)| faction.name == spawn.faction)
                .unwrap()
                .0;

            // Retrieve the unit definition by name.
            let unit_def = unit_defs.iter().find(|def| def.name == spawn.unit).unwrap();

            // Create the unit based on the definition.
            let unit = Unit {
                name: spawn.name,
                role: spawn.role,
                faction: faction_id,
                health: unit_def.health,
                armour: unit_def.armour,
                shield: unit_def.shield,
                stamina: unit_def.stamina,
                speed: unit_def.speed,
            };

            // Insert unit into faction's unit table.
            let unit_id = game.units.insert(unit);
            let faction = game.factions.get_mut(faction_id).unwrap();
            faction.units.insert(unit_id);
        }
    }

    let mut engine = Cherry::new("Foo, Bar, Baz!", 60, 40, "res/fonts/default.png");
    engine.run(&mut game);
}
