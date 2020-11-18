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
    pub state: GameState,

    // World
    pub units: Vec<Unit>,

    // Data caches
    pub selected_unit: Option<u32>,
    pub selectable: Vec<u32>,

    // Menus
    pub item_index: usize,
    pub item_count: usize,
    pub commands_menu: Menu<Commands>,
    pub select_menu: Menu<Select>,
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

        // Input
        if engine.key(Key::Up).just_down {
            if self.item_count != 0 && self.item_index != 0 {
                self.item_index -= 1;
            }
        }

        if engine.key(Key::Down).just_down {
            if self.item_count != 0 && self.item_index < self.item_count - 1 {
                self.item_index += 1;
            }
        }

        // Update data caches.
        self.selectable.clear();

        for unit in &self.units {
            if unit.faction == 0 && unit.health != 0 {
                self.selectable.push(unit.id);
            }
        }

        if let Some(id) = self.selected_unit {
            let unit = &self.units[id as usize];
            engine.set_fg(Colour::VERY_DARK_CYAN);
            engine.draw_str(20, 1, &format!("Selected: {}", unit.name));
        }

        // Handle state.
        match self.state {
            GameState::CommandsMenu => {
                // Validate commands menu.
                self.commands_menu.clear();

                if self.selectable.len() != 0 {
                    self.commands_menu.add("Select", Commands::Select);
                }

                self.commands_menu.add("End Turn", Commands::EndTurn);
                self.item_count = self.commands_menu.len();

                // Draw.
                draw_menu(engine, 1, 1, &self.commands_menu, self.item_index);

                // Input.
                if engine.key(Key::Enter).just_down {
                    let item = self.commands_menu.get(self.item_index).unwrap();
                    let command = item.data();
                    match command {
                        Commands::Select => {
                            self.state = GameState::SelectMenu;
                            self.item_index = 0;
                        }
                        Commands::Attack => {}
                        Commands::Move => {}
                        Commands::EndTurn => {}
                    }
                }
            }

            GameState::SelectMenu => {
                // Set menu items.
                self.select_menu.clear();

                for id in &self.selectable {
                    let unit = &self.units[*id as usize];
                    self.select_menu.add(&unit.name, Select::Unit { id: *id });
                }

                self.item_count = self.select_menu.len();

                // Draw.
                draw_menu(engine, 1, 1, &self.select_menu, self.item_index);
                
                // Input.
                if engine.key(Key::Enter).just_down {
                    let item = self.select_menu.get(self.item_index).unwrap();
                    match item.data() {
                        Select::Unit { id } => {
                            self.state = GameState::CommandsMenu;
                            self.selected_unit = Some(*id);
                            self.item_index = 0;
                        }
                    }
                }
            }
        }
    }
}

pub enum GameState {
    CommandsMenu,
    SelectMenu,
}

pub enum Commands {
    Select,
    Attack,
    Move,
    EndTurn,
}

pub enum Select {
    Unit { id: u32 },
}

fn main() {
    let mut game = Game {
        state: GameState::CommandsMenu,
        units: Vec::new(),
        selected_unit: None,
        selectable: Vec::new(),
        item_index: 0,
        item_count: 0,
        commands_menu: Menu::new("COMMANDS"),
        select_menu: Menu::new("SELECT"),
    };

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
