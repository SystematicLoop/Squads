use crate::graphics::colour::Colour;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub glyph: char,
    pub fg: Colour,
    pub bg: Colour,
}

impl Tile {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            glyph: ' ',
            fg: Colour::WHITE,
            bg: Colour::BLACK,
        }
    }
}
