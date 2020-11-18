pub mod engine;
pub mod event;
pub mod graphics;
pub mod input;
pub mod terminal;
pub mod window;

use engine::Engine;

pub trait Cherry {
    fn on_update(&mut self, engine: &mut Engine);
}
