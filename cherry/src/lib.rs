pub mod chars;
pub mod engine;
pub mod event;
pub mod graphics;
pub mod input;
pub mod terminal;
pub mod window;

use engine::Cherry;

pub trait CherryApp {
    fn on_update(&mut self, engine: &mut Cherry);
}
