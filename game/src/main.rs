pub mod gui;
pub mod unit;

use std::collections::VecDeque;

use cherry::{
    engine::Cherry,
    graphics::colour::Colour,
    input::key::Key,
    CherryApp,
};

use gui::{
    draw_stat,
    info::draw_unit_info,
    menu::{
        draw_menu,
        Menu,
    },
    messages::draw_messages,
};

use unit::Unit;

#[derive(Default)]
pub struct Game {
    // World
    pub units: Vec<Unit>,

    // Data caches
    pub current_faction_id: u32,
    pub selected_unit_id: Option<u32>,
    pub target_unit_id: Option<u32>,

    pub selectable: Vec<u32>,
    pub targetable: Vec<u32>,

    pub messages: VecDeque<String>,

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
    fn change_menu(&mut self, menu_id: usize, force_menu_changed: bool) {
        if self.menu_id != menu_id || force_menu_changed {
            self.menu_id = menu_id;
            self.item_id = 0;
            self.menu_changed = true;
        }
    }

    fn record_message(&mut self, message: &str) {
        self.messages.push_front(String::from(message));

        // Todo: Remove this magic number.
        while self.messages.len() >= 10 {
            self.messages.pop_back();
        }
    }

    fn update_commands_menu(&mut self) {
        // Update selectable units.
        self.selectable.clear();

        for unit in &self.units {
            if let Some(id) = self.selected_unit_id {
                if unit.id == id {
                    continue;
                }
            }

            if unit.faction == 0 && unit.health != 0 {
                self.selectable.push(unit.id);
            }
        }

        // Update targetable units.
        self.targetable.clear();

        if let Some(selected_unit_id) = self.selected_unit_id {
            let unit = &self.units[selected_unit_id as usize];

            for target in &self.units {
                if target.faction != unit.faction && target.health != 0 {
                    self.targetable.push(target.id);
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

    fn update_select_menu(&mut self) {
        // Update menu items.
        let menu = &mut self.menus[self.select_menu_id];
        menu.clear();

        for id in &self.selectable {
            let unit = &self.units[*id as usize];
            menu.add(&unit.name, MenuData::SelectUnit { id: *id });
        }

        menu.add(
            "Back",
            MenuData::ChangeMenu {
                id: self.commands_menu_id,
            },
        );
    }

    fn update_attack_menu(&mut self) {
        // Update menu items.
        let menu = &mut self.menus[self.attack_menu_id];
        menu.clear();

        for id in &self.targetable {
            let unit = &self.units[*id as usize];
            menu.add(
                &unit.name,
                MenuData::AttackUnit {
                    id: *id,
                },
            );
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
                    let unit = &self.units[*id as usize];
                    draw_unit_info(engine, &unit, 1, 14, 26, 16);
                }
                _ => {}
            }
        } else if self.menu_id == self.attack_menu_id {
            let menu = &self.menus[self.menu_id];
                let item = menu.get(self.item_id).unwrap();
    
                match item.data() {
                    MenuData::AttackUnit { id } => {
                        let unit = &self.units[*id as usize];
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
                        self.change_menu(id, false);
                    }
                    MenuData::SelectUnit { id } => {
                        self.selected_unit_id = Some(id);
                        self.change_menu(self.commands_menu_id, false);

                        let unit = &self.units[id as usize];
                        let message = format!("Selected {}.", unit.name);
                        self.record_message(&message);
                    }
                    MenuData::DeselectUnit => {
                        if let Some(id) = self.selected_unit_id {
                            let unit = &self.units[id as usize];
                            let message = format!("Deselected {}.", unit.name);

                            self.selected_unit_id = None;
                            self.change_menu(self.commands_menu_id, true);
                            self.record_message(&message);
                        }
                    }
                    MenuData::AttackUnit { id } => {
                        let unit = &self.units[self.selected_unit_id.unwrap() as usize];
                        let target_unit = &self.units[id as usize];
                        let message = format!("{} attacked {}.", unit.name, target_unit.name);
                        self.record_message(&message);

                        let mut target_unit = &mut self.units[id as usize];
                        target_unit.health = 0;
                        let message = format!("{} was killed.", target_unit.name);
                        self.record_message(&message);

                        self.change_menu(self.commands_menu_id, false);
                    }
                    MenuData::EndTurn => {
                        self.record_message("You end the turn.");
                    }
                    MenuData::Empty => {
                        self.record_message("Nothing happens.");
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MenuData {
    ChangeMenu { id: usize },
    SelectUnit { id: u32 },
    AttackUnit { id: u32 },
    DeselectUnit,
    EndTurn,
    Empty,
}

fn main() {
    let mut game = Game::default();

    {
        game.commands_menu_id = game.menus.len();
        game.menus.push(Menu::new("COMMANDS"));

        game.select_menu_id = game.menus.len();
        game.menus.push(Menu::new("SELECT"));

        game.attack_menu_id = game.menus.len();
        game.menus.push(Menu::new("ATTACK"));

        game.change_menu(game.commands_menu_id, true);
    }

    {
        game.units.push(Unit {
            id: 0,
            name: String::from("John"),
            faction: 0,
            health: 10,
            health_max: 10,
            armour: 10,
            armour_max: 10,
            shield: 10,
            shield_max: 10,
            actions: 100,
            speed: 50,
        });

        game.units.push(Unit {
            id: 1,
            name: String::from("Mary"),
            faction: 0,
            health: 8,
            health_max: 10,
            armour: 0,
            armour_max: 0,
            shield: 5,
            shield_max: 5,
            actions: 100,
            speed: 50,
        });

        game.units.push(Unit {
            id: 2,
            name: String::from("Sue"),
            faction: 0,
            health: 1,
            health_max: 10,
            armour: 0,
            armour_max: 0,
            shield: 0,
            shield_max: 0,
            actions: 100,
            speed: 50,
        });

        game.units.push(Unit {
            id: 3,
            name: String::from("Doe"),
            faction: 1,
            health: 12,
            health_max: 12,
            armour: 0,
            armour_max: 0,
            shield: 0,
            shield_max: 0,
            actions: 100,
            speed: 50,
        });
    }

    let mut engine = Cherry::new("Foo, Bar, Baz!", 60, 40, "res/fonts/default.png");
    engine.run(&mut game);
}
