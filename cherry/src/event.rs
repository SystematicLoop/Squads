use crate::input::{
    button::Button,
    key::Key,
};

pub enum Event {
    KeyDown { key: Key },
    KeyUp { key: Key },
    MouseMove { x: i32, y: i32, dx: i32, dy: i32 },
    MouseButtonDown { button: Button },
    MouseButtonUp { button: Button },
    MouseScroll { delta: i32 },
    Quit,
}
