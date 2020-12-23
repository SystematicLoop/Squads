use std::path::{Path, PathBuf};

use super::colour::Colour;
use blueberry::Grid;

pub struct Sprite {
    path: PathBuf,
    grid: Grid<Colour>,
}

impl Sprite {
    pub fn load<P>(path: P) -> Result<Self, image::ImageError>
    where
        P: AsRef<Path>
    {
        let path = PathBuf::from(path.as_ref());
        let image_buffer = image::open(&path)?;
        let image = image_buffer.to_rgba8();

        let width = image.width();
        let height = image.height();
        let size = width * height;

        let mut data = Vec::with_capacity(size as usize);

        for row in image.rows() {
            for pixel in row {
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];

                data.push(Colour::new(r, g, b));
            }
        }

        let grid = Grid::from_slice(&data, width, height);

        let sprite = Sprite {
            path,
            grid,
        };

        Ok(sprite)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn width(&self) -> u32 {
        self.grid.columns()
    }

    pub fn height(&self) -> u32 {
        self.grid.rows()
    }

    pub fn area(&self) -> u32 {
        self.grid.size()
    }

    pub fn get(&self, index: usize) -> Option<&Colour> {
        self.grid.get(index)
    }

    pub fn get_at(&self, x: i32, y: i32) -> Option<&Colour> {
        self.grid.get_at(x, y)
    }

    pub fn data(&self) -> &[Colour] {
        self.grid.data()
    }
}
