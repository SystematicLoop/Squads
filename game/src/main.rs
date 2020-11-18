pub mod gui;

use cherry::{
    engine::Engine,
    graphics::colour::Colour,
    input::key::Key,
    Cherry,
};

use gui::commands_menu::CommandsMenu;

pub struct Game {
    pub selectable: Vec<u32>,
    pub commands_menu: CommandsMenu,
}

impl Cherry for Game {
    fn on_update(&mut self, engine: &mut Engine) {
        engine.set_fg(Colour::WHITE);
        engine.set_bg(Colour::BLACK);
        engine.clear();
        engine.set_fg(Colour::new(16, 16, 16));
        engine.draw_border(0, 0, 60, 40);

        let result = gui::commands_menu::commands_menu(self, engine, self.commands_menu);
        if result.chosen {
            println!("Chose {:#?}", result.select);
        }

        self.commands_menu = result.select;
    }
}

fn main() {
    let mut game = Game {
        selectable: Vec::new(),
        commands_menu: CommandsMenu::Select,
    };

    game.selectable.push(69);

    let mut engine = Engine::new(
        "Foo, Bar, Baz!",
        60,
        40,
        "res/fonts/cp437_14x14.png",
    );
    engine.run(&mut game);
}
