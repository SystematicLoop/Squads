use super::{
    opengl::texture::Texture,
    sprite::Sprite,
};

pub struct Font {
    width: u32,
    height: u32,
    texture: Texture,
}

impl Font {
    pub fn new(sprite: &Sprite) -> Self {
        let width = sprite.width() / 16;
        let height = sprite.height() / 16;
        let texture = Texture::new(sprite.width(), sprite.height(), sprite.data());

        Self {
            width,
            height,
            texture,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}
