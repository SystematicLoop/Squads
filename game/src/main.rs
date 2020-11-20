pub mod gui;
pub mod unit;

use cherry::{
    engine::Engine,
    graphics::colour::Colour,
    input::key::Key,
    Cherry,
};

use gui::menu::{
    draw_menu,
    Menu,
};

use unit::Unit;

pub struct Game {
    // World
    pub units: Vec<Unit>,

    // Data caches
    pub selected_unit: Option<u32>,
    pub selectable: Vec<u32>,

    // Menus
    pub menus: Vec<Menu<MenuData>>,
    pub menu_id: usize,
    pub item_id: usize,
    pub menu_changed: bool,
    pub commands_menu_id: usize,
    pub select_menu_id: usize,
}


impl Game {
    pub fn change_menu(&mut self, menu_id: usize) {
        if self.menu_id != menu_id {
            self.menu_id = menu_id;
            self.item_id = 0;
            self.menu_changed = true;
        }
    }
}

impl Cherry for Game {
    fn on_update(&mut self, engine: &mut Engine) {
        // Clear view.
        engine.set_fg(Colour::WHITE);
        engine.set_bg(Colour::BLACK);
        engine.clear();

        // Draw border.
        engine.set_fg(Colour::new(16, 16, 16));
        engine.draw_border(0, 0, 60, 40);

        // Update menu.
        if self.menu_changed {
            self.menu_changed = false;

            if self.menu_id == self.commands_menu_id {
                // Update selectable units.
                self.selectable.clear();
    
                for unit in &self.units {
                    if unit.faction == 0 && unit.health != 0 {
                        self.selectable.push(unit.id);
                    }
                }

                // Update menu items.
                let menu = &mut self.menus[self.commands_menu_id];
                menu.clear();
                
                if self.selectable.len() != 0 {
                    menu.add("Select", MenuData::ChangeMenu { id: self.select_menu_id });
                }

                menu.add("End Turn", MenuData::Empty);
            }
    
            if self.menu_id == self.select_menu_id {
                // Update menu items.
                let menu = &mut self.menus[self.select_menu_id];
                menu.clear();

                for id in &self.selectable {
                    let unit = &self.units[*id as usize];
                    menu.add(&unit.name, MenuData::SelectUnit { id: *id });
                }

                menu.add("Back", MenuData::ChangeMenu { id: self.commands_menu_id });
            }
        }

        // Draw menu.
        let menu = &self.menus[self.menu_id];
        draw_menu(engine, 1, 1, &menu, self.item_id);

        // Input.
        if engine.key(Key::Up).just_down {
            self.item_id = self.item_id.saturating_sub(1);
        }

        if engine.key(Key::Down).just_down {
            self.item_id = (self.item_id + 1).min(menu.len());
        }

        if engine.key(Key::Enter).just_down {
            if let Some(item) = menu.get(self.item_id) {
                let data = item.data().clone();
                match data {
                    MenuData::ChangeMenu { id } => {
                        self.change_menu(id);
                    }
                    MenuData::SelectUnit { id } => {
                        self.selected_unit = Some(id);
                        self.change_menu(self.commands_menu_id);
                    }
                    MenuData::EndTurn => {}
                    MenuData::Empty => {}
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MenuData {
    ChangeMenu { id: usize },
    SelectUnit { id: u32 },
    EndTurn,
    Empty,
}

fn main() {
    let mut game = Game {
        units: Vec::new(),
        selected_unit: None,
        selectable: Vec::new(),
        menus: Vec::new(),
        menu_changed: true,
        menu_id: 0,
        item_id: 0,
        commands_menu_id: 0,
        select_menu_id: 0,
    };

    {
        game.commands_menu_id = game.menus.len();
        game.menus.push(Menu::new("COMMANDS"));

        game.select_menu_id = game.menus.len();
        game.menus.push(Menu::new("SELECT"));
    }

    {
        game.units.push(Unit {
            id: 0,
            name: String::from("John"),
            faction: 0,
            health: 10,
            health_max: 10,
        });

        game.units.push(Unit {
            id: 1,
            name: String::from("Mary"),
            faction: 0,
            health: 8,
            health_max: 10,
        });

        game.units.push(Unit {
            id: 2,
            name: String::from("Sue"),
            faction: 0,
            health: 0,
            health_max: 10,
        });

        game.units.push(Unit {
            id: 3,
            name: String::from("Doe"),
            faction: 1,
            health: 12,
            health_max: 12,
        });
    }

    let mut engine = Engine::new("Foo, Bar, Baz!", 60, 40, "res/fonts/cp437_14x14.png");
    engine.run(&mut game);
}
