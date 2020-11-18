use cherry::{
    engine::Engine,
    graphics::colour::Colour,
    input::key::Key,
    Cherry,
};

struct Game;

impl Cherry for Game {
    fn on_update(&mut self, engine: &mut cherry::engine::Engine) {
        engine.set_fg(Colour::WHITE);
        engine.set_bg(Colour::BLACK);
        engine.clear();
        engine.draw_str(0, 0, "Hello, world!");

        if engine.key(Key::Enter).held {
            engine.set_fg(Colour::RED);
            engine.draw_str(0, 1, "Booo!");
        }
    }
}

fn main() {
    let mut game = Game;
    let mut engine = Engine::new("Foo, Bar, Baz!", 60, 40, "res/fonts/default.png");
    engine.run(&mut game);
}
