#![feature(split_inclusive)]

pub mod commands;
pub mod faction;
pub mod gid;
pub mod gui;
pub mod icons;
pub mod logger;
pub mod scenario;
pub mod serde;
pub mod unit;
pub mod weapon;
pub mod world;

use std::{
    collections::{
        HashSet,
        VecDeque,
    },
    fs::File,
    io::{
        LineWriter,
        Write,
    },
    panic,
    path::PathBuf,
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
    info::{
        draw_area_info,
        draw_unit_info,
        draw_weapon_info,
    },
    menu::{
        draw_menu,
        Menu,
    },
    messages::draw_messages,
};

use logger::Logger;
use scenario::Scenario;

use unit::{
    Stat,
    Unit,
    UnitDef,
    UnitSpawn,
};

use weapon::{
    Ammo,
    Weapon,
    WeaponDef,
};
use world::{
    Area,
    AreaDef,
    Edge,
    World,
};

#[derive(Default)]
pub struct Game {
    // World
    pub factions: Arena<Faction>,
    pub units: Arena<Unit>,
    pub weapons: Arena<Weapon>,
    pub areas: Arena<Area>,
    pub world: World,

    // Data caches
    pub current_faction_id: Option<Gid>,
    pub turn_queue: VecDeque<Gid>,

    pub selected_unit_id: Option<Gid>,
    pub info_unit_id: Option<Gid>,
    pub info_target_id: Option<Gid>,
    pub info_area_id: Option<Gid>,

    pub selectable: Vec<Gid>,
    pub targetable: Vec<Gid>,
    pub reachable: Vec<Gid>,

    pub show_map: bool,

    pub logger: Logger,

    // Menus
    pub menus: Vec<Menu<MenuData>>,
    pub menu_id: usize,
    pub item_id: usize,
    pub menu_changed: bool,
    pub commands_menu_id: usize,
    pub select_menu_id: usize,
    pub attack_menu_id: usize,
    pub move_menu_id: usize,
}

impl Game {
    pub fn change_menu(&mut self, menu_id: usize, force_menu_changed: bool) {
        if self.menu_id != menu_id || force_menu_changed {
            self.menu_id = menu_id;
            self.item_id = 0;
            self.menu_changed = true;
        }
    }

    pub fn update_commands_menu(&mut self) {
        // Reset data caches.
        self.selectable.clear();
        self.targetable.clear();
        self.reachable.clear();

        let mut can_reload = false;

        if let Some(faction_id) = self.current_faction_id {
            // We assume the current faction is valid.
            let faction = self.factions.get(faction_id).unwrap();

            // Update selectable units.
            for unit_id in &faction.units {
                let unit = &self.units[*unit_id];

                if unit.health.val() != 0 {
                    self.selectable.push(*unit_id);
                }
            }

            if let Some(selected_unit_id) = self.selected_unit_id {
                // Update targetable units.
                let unit = &self.units[selected_unit_id];
                let weapon = &self.weapons[unit.weapon_id];

                if unit.stamina.full() && weapon.ammo.val != 0 {
                    for (target_id, target) in &self.units {
                        if target.faction_id != unit.faction_id
                            && target.area_id == unit.area_id
                            && target.health.val() != 0
                        {
                            self.targetable.push(target_id);
                        }
                    }
                }

                if unit.stamina.val() >= 25 && weapon.ammo.val < weapon.ammo.max {
                    can_reload = true;
                }

                // Update reachable areas.
                if unit.stamina.val() >= 50 {
                    let edges = self.world.edges(unit.area_id);
                    for (_from, to, _edge) in edges {
                        self.reachable.push(to);
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
                    menu_id: self.select_menu_id,
                },
            );
        }

        if self.targetable.len() != 0 {
            menu.add(
                "Attack",
                MenuData::ChangeMenu {
                    menu_id: self.attack_menu_id,
                },
            );
        }

        if can_reload {
            menu.add(
                "Reload",
                MenuData::Reload {
                    unit_id: self.selected_unit_id.unwrap(),
                },
            );
        }

        if self.reachable.len() != 0 {
            menu.add(
                "Move",
                MenuData::ChangeMenu {
                    menu_id: self.move_menu_id,
                },
            );
        }

        if !self.turn_queue.is_empty() {
            menu.add("End Turn", MenuData::EndTurn);
        }
    }

    pub fn update_select_menu(&mut self) {
        // Update menu items.
        let menu = &mut self.menus[self.select_menu_id];
        menu.clear();

        for unit_id in &self.selectable {
            let unit = &self.units[*unit_id];
            let is_selected = self.selected_unit_id == Some(*unit_id);
            let label = if is_selected {
                format!("{} *", unit.name)
            } else {
                format!("{}", unit.name)
            };

            menu.add(&label, MenuData::SelectUnit { unit_id: *unit_id })
        }

        menu.add(
            "Back",
            MenuData::ChangeMenu {
                menu_id: self.commands_menu_id,
            },
        );
    }

    pub fn update_attack_menu(&mut self) {
        // Update menu items.
        let menu = &mut self.menus[self.attack_menu_id];
        menu.clear();

        for target_id in &self.targetable {
            let target = &self.units[*target_id];
            menu.add(
                &target.name,
                MenuData::AttackUnit {
                    target_id: *target_id,
                },
            );
        }

        menu.add(
            "Back",
            MenuData::ChangeMenu {
                menu_id: self.commands_menu_id,
            },
        )
    }

    pub fn update_move_menu(&mut self) {
        // Update menu items.
        let menu = &mut self.menus[self.move_menu_id];
        menu.clear();

        let selected_unit_id = self.selected_unit_id.unwrap();

        for area_id in &self.reachable {
            let area = &self.areas[*area_id];
            menu.add(
                &area.name,
                MenuData::MoveUnit {
                    unit_id: selected_unit_id,
                    destination_id: *area_id,
                },
            );
        }

        menu.add(
            "Back",
            MenuData::ChangeMenu {
                menu_id: self.commands_menu_id,
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
            } else if self.menu_id == self.select_menu_id {
                self.update_select_menu();
            } else if self.menu_id == self.attack_menu_id {
                self.update_attack_menu();
            } else if self.menu_id == self.move_menu_id {
                self.update_move_menu();
            } else {
                panic!("Menu {} not registered to update.", self.menu_id);
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

        // Refresh the infos.
        self.info_unit_id = self.selected_unit_id;
        self.info_target_id = None;
        self.info_area_id = None;

        if self.menu_id == self.select_menu_id {
            let menu = &self.menus[self.menu_id];
            let item = menu.get(self.item_id).unwrap();

            match item.data() {
                MenuData::SelectUnit { unit_id } => {
                    self.info_unit_id = Some(*unit_id);
                }
                _ => {
                    self.info_unit_id = None;
                }
            }
        } else if self.menu_id == self.attack_menu_id {
            let menu = &self.menus[self.menu_id];
            let item = menu.get(self.item_id).unwrap();

            match item.data() {
                MenuData::AttackUnit { target_id } => {
                    self.info_target_id = Some(*target_id);
                }
                _ => {
                    self.info_target_id = None;
                }
            }
        } else if self.menu_id == self.move_menu_id {
            let menu = &self.menus[self.menu_id];
            let item = menu.get(self.item_id).unwrap();

            match item.data() {
                MenuData::MoveUnit { destination_id, .. } => {
                    self.info_area_id = Some(*destination_id);
                }
                _ => {
                    self.info_area_id = None;
                }
            }
        }

        // Draw infos.
        if let Some(unit_id) = self.info_unit_id {
            let unit = self.units.get(unit_id).unwrap();
            let selected = self.info_unit_id == self.selected_unit_id;

            draw_unit_info(self, engine, unit_id, selected, 1, 14, 25, 25);
            draw_weapon_info(self, engine, unit.weapon_id, 1, 31, 25, 8);
        }

        if let Some(target_id) = self.info_target_id {
            let target = self.units.get(target_id).unwrap();

            draw_unit_info(self, engine, target_id, false, 27, 14, 32, 25);
            draw_weapon_info(self, engine, target.weapon_id, 27, 31, 32, 8);
        } else if let Some(area_id) = self.info_area_id {
            draw_area_info(self, engine, area_id, 27, 14, 32, 25);
        }

        // Draw messages.
        if engine.key(Key::L).held {
            draw_messages(engine, 27, 1, 32, 39, &self.logger.messages);
        } else {
            draw_messages(engine, 27, 1, 32, 13, &self.logger.messages);
        }

        // Save log.
        if engine.key(Key::P).just_down {
            if let Ok(file) = File::create("log.txt") {
                let mut writer = LineWriter::new(file);

                let mut buffer = String::new();
                for message in &self.logger.messages {
                    for token in &message.tokens {
                        buffer += &token.content;
                    }

                    buffer += "\n";
                    writer.write_all(buffer.as_bytes()).unwrap();
                    buffer.clear();
                }
            } else {
                println!("Failed to save log to disk.");
            }
        }

        // Debug.
        if engine.key(Key::Backspace).held {
            let (mx, my) = engine.mouse_pos();
            engine.set_fg(Colour::RED);
            engine.draw(mx, my, 'X');

            engine.set_fg(Colour::WHITE);
            engine.draw_str(0, 0, &format!("({},{})", mx, my));
        }

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

        if engine.key(Key::M).just_down {
            self.show_map = !self.show_map;
        }

        if engine.key(Key::Enter).just_down {
            if let Some(item) = menu.get(self.item_id) {
                let data = item.data().clone();
                match data {
                    MenuData::ChangeMenu { menu_id } => {
                        self.change_menu(menu_id, true);
                    }
                    MenuData::SelectUnit { unit_id } => {
                        commands::select(self, unit_id);
                    }
                    MenuData::AttackUnit { target_id } => {
                        commands::attack(self, target_id);
                    }
                    MenuData::Reload { unit_id } => {
                        commands::reload(self, unit_id);
                    }
                    MenuData::MoveUnit {
                        unit_id,
                        destination_id,
                    } => {
                        commands::movement(self, unit_id, destination_id);
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

#[derive(Debug, Copy, Clone)]
pub enum MenuData {
    ChangeMenu { menu_id: usize },
    SelectUnit { unit_id: Gid },
    AttackUnit { target_id: Gid },
    Reload { unit_id: Gid },
    MoveUnit { unit_id: Gid, destination_id: Gid },
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

        game.move_menu_id = game.menus.len();
        game.menus.push(Menu::new("MOVE"));

        game.change_menu(game.commands_menu_id, true);
    }

    // Load scenario.
    let scenario_name: String = serde::deserialise_file("res/scenarios/startup.json");
    let scenario_dir_path = PathBuf::from("res/scenarios/").join(scenario_name);
    let scenario_file_path = scenario_dir_path.join("scenario.json");
    let areas_file_path = scenario_dir_path.join("areas.json");
    let map_file_path = scenario_dir_path.join("map.png");
    let factions_file_path = scenario_dir_path.join("factions.json");
    let weapons_file_path = scenario_dir_path.join("weapons.json");
    let units_file_path = scenario_dir_path.join("units.json");
    let connections_file_path = scenario_dir_path.join("connections.json");
    let spawns_file_path = scenario_dir_path.join("spawns.json");

    let scenario: Scenario = serde::deserialise_file(scenario_file_path);

    // Load world.
    {
        let area_defs: Vec<AreaDef> = serde::deserialise_file(areas_file_path);

        for area_def in area_defs {
            let area = Area {
                name: area_def.name.clone(),
                units: HashSet::new(),
            };

            let area_id = game.areas.insert(area);
            game.world.add_node(area_id);
        }

        let edges: Vec<(String, String)> = serde::deserialise_file(connections_file_path);
        for (from, to) in edges {
            let from_id = game
                .areas
                .iter()
                .find(|(_id, area)| area.name == from)
                .unwrap()
                .0;

            let to_id = game
                .areas
                .iter()
                .find(|(_id, area)| area.name == to)
                .unwrap()
                .0;

            game.world.add_edge(from_id, to_id, Edge);
        }
    }

    // Load factions.
    {
        let faction_defs: Vec<FactionDef> = serde::deserialise_file(factions_file_path);
        for faction_def in faction_defs {
            let faction = Faction {
                name: faction_def.name,
                units: HashSet::new(),
            };

            let is_starting_faction = faction.name == scenario.starting_faction;
            let faction_id = game.factions.insert(faction);

            if is_starting_faction {
                game.current_faction_id = Some(faction_id);
            } else {
                game.turn_queue.push_back(faction_id);
            }
        }

        assert!(game.current_faction_id.is_some());
        game.turn_queue.push_back(game.current_faction_id.unwrap());
    }

    // Load units.
    {
        let weapon_defs: Vec<WeaponDef> = serde::deserialise_file(weapons_file_path);
        let unit_defs: Vec<UnitDef> = serde::deserialise_file(units_file_path);
        let spawns: Vec<UnitSpawn> = serde::deserialise_file(spawns_file_path);

        for spawn in spawns {
            // Retrieve the faction by name.
            let faction_id = game
                .factions
                .iter()
                .find(|(_id, faction)| faction.name == spawn.faction)
                .unwrap()
                .0;

            // Retrieve the area by name.
            let area_id = game
                .areas
                .iter()
                .find(|(_id, area)| area.name == spawn.area)
                .unwrap()
                .0;

            // Retrieve the weapon definition by name.
            let weapon_def = weapon_defs
                .iter()
                .find(|def| def.name == spawn.weapon)
                .unwrap();

            // Create the weapon based on the definition.
            let weapon = Weapon {
                name: weapon_def.name.clone(),
                role: weapon_def.role.clone(),
                ammo: Ammo {
                    val: weapon_def.ammo,
                    max: weapon_def.ammo,
                },
                accuracy: weapon_def.accuracy,
                rolls: weapon_def.rolls,
                weight: weapon_def.weight,
                damage: weapon_def.damage,
            };

            // Insert the weapon into the world.
            let weapon_id = game.weapons.insert(weapon);

            // Retrieve the unit definition by name.
            let unit_def = unit_defs.iter().find(|def| def.name == spawn.unit).unwrap();

            // Create the unit based on the definition.
            let unit = Unit {
                name: spawn.name,
                role: spawn.role,
                faction_id,
                area_id,
                accuracy: unit_def.accuracy,
                weapon_id,
                health: Stat::new(unit_def.health),
                armour: Stat::new(unit_def.armour),
                shield: Stat::new(unit_def.shield),
                stamina: Stat::new(100),
                speed: unit_def.speed,
            };

            // Insert the unit into the world.
            let unit_id = game.units.insert(unit);

            // Insert unit into faction's unit table.
            let faction = game.factions.get_mut(faction_id).unwrap();
            faction.units.insert(unit_id);

            // Insert unit into unit's unit table.
            let area = game.areas.get_mut(area_id).unwrap();
            area.units.insert(unit_id);
        }
    }

    let mut engine = Cherry::new("Foo, Bar, Baz!", 60, 40, "res/fonts/default.png");
    engine.run(&mut game);
}
