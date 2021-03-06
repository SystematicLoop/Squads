use blueberry::Vec2f;

use crate::{
    event::Event,
    graphics::colour::Colour,
    graphics::sprite::Sprite,
    graphics::{
        font::Font,
        opengl::renderer::Renderer,
    },
    input::button::Button,
    input::key::Key,
    terminal::{
        buffer::Buffer,
        tile::Tile,
    },
    window::Window,
    Cherry,
};

pub struct Engine {
    window: Window,

    // Graphics
    font: Font,
    renderer: Renderer,
    buffer: Buffer,
    fg: Colour,
    bg: Colour,

    // Input
    mx: i32,
    my: i32,
    md: i32,
    buttons_last_frame: [bool; Button::count() + 1],
    buttons_this_frame: [bool; Button::count() + 1],
    keys_last_frame: [bool; Key::count() + 1],
    keys_this_frame: [bool; Key::count() + 1],
}

impl Engine {
    pub fn new(title: &str, columns: u32, rows: u32, font: &str) -> Self {
        let font_sprite = Sprite::load(font).expect("Failed to load font.");

        let font_width = font_sprite.width() / 16;
        let font_height = font_sprite.height() / 16;

        let window = Window::new(title, columns * font_width, rows * font_height);
        let font = Font::new(&font_sprite);

        let renderer = Renderer::new(columns, rows);
        let buffer = Buffer::filled(Tile::default(), columns, rows);

        Self {
            window,
            font,
            renderer,
            buffer,
            fg: Colour::WHITE,
            bg: Colour::BLACK,
            mx: 0,
            my: 0,
            md: 0,
            buttons_last_frame: [false; Button::count() + 1],
            buttons_this_frame: [false; Button::count() + 1],
            keys_this_frame: [false; Key::count() + 1],
            keys_last_frame: [false; Key::count() + 1],
        }
    }

    pub fn run(&mut self, client: &mut dyn Cherry) {
        let mut running = true;
        while running {
            //----------------------------------------------------------------
            // Update input state.
            self.buttons_last_frame
                .copy_from_slice(&self.buttons_this_frame);
            self.keys_last_frame.copy_from_slice(&self.keys_this_frame);
            self.md = 0;

            //----------------------------------------------------------------
            // Process events.
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::MouseButtonDown { button } => {
                        self.buttons_this_frame[button as usize] = true;
                    }
                    Event::MouseButtonUp { button } => {
                        self.buttons_this_frame[button as usize] = false;
                    }
                    Event::KeyDown { key } => {
                        self.keys_this_frame[key as usize] = true;
                    }
                    Event::KeyUp { key } => {
                        self.keys_this_frame[key as usize] = false;
                    }
                    Event::MouseMove { x, y, .. } => {
                        self.mx = x / 8;
                        self.my = y / 12;
                    }
                    Event::MouseScroll { delta } => {
                        self.md = delta;
                    }
                    Event::Quit => {
                        running = false;
                    }
                }
            }

            //----------------------------------------------------------------
            // Callback to client.
            client.on_update(self);

            //----------------------------------------------------------------
            // Render.
            unsafe {
                gl::ClearColor(0.08, 0.08, 0.08, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.renderer
                .draw_buffer(&self.font, Vec2f::zero(), &mut self.buffer);
            self.window.swap_buffers();
        }
    }

    pub fn set_font(&mut self, path: &str) {
        // Load font.
        let font_sprite = Sprite::load(path).expect("Failed to load font.");
        let font = Font::new(&font_sprite);

        // Update viewport.
        let width = self.buffer.columns() * font.width();
        let height = self.buffer.rows() * font.height();
        self.window.set_size(width, height);

        // Update font.
        self.font = font;
    }

    pub fn button(&self, button: Button) -> ButtonState {
        let last_frame = self.buttons_last_frame[button as usize];
        let this_frame = self.buttons_this_frame[button as usize];

        let just_down = this_frame && !last_frame;
        let just_up = !this_frame && last_frame;
        let held = this_frame && last_frame;

        ButtonState {
            just_down,
            just_up,
            held,
        }
    }

    pub fn key(&self, key: Key) -> KeyState {
        let last_frame = self.keys_last_frame[key as usize];
        let this_frame = self.keys_this_frame[key as usize];

        let just_down = !last_frame && this_frame;
        let just_up = last_frame && !this_frame;
        let held = (!last_frame && this_frame) || (last_frame && this_frame);

        KeyState {
            just_down,
            just_up,
            held,
        }
    }

    pub fn scroll(&self) -> i32 {
        self.md
    }

    pub fn clear(&mut self) {
        for tile in self.buffer.data_mut().iter_mut() {
            tile.glyph = ' ';
            tile.fg = Colour::WHITE;
            tile.bg = self.bg;
        }
    }

    pub fn set_fg(&mut self, fg: Colour) {
        self.fg = fg;
    }

    pub fn set_bg(&mut self, bg: Colour) {
        self.bg = bg;
    }

    pub fn draw(&mut self, x: i32, y: i32, c: char) {
        // Check for out-of-bounds.
        let columns = self.buffer.columns() as i32;
        let rows = self.buffer.rows() as i32;
        if x < 0 || x >= columns || y < 0 || y >= rows {
            // The coordinates are out-of-bounds!
            return;
        }

        let index = (x + y * columns) as usize;
        let tile = &mut self.buffer.get_mut(index).unwrap();
        tile.glyph = c;
        tile.fg = self.fg;
        tile.bg = self.bg;
    }

    pub fn draw_h_line(&mut self, x: i32, y: i32, w: i32, c: char) {
        let x0 = x;
        let x1 = x + w;

        for x in x0..x1 {
            self.draw(x, y, c);
        }
    }

    pub fn draw_str(&mut self, x: i32, y: i32, str: &str) {
        for (i, c) in str.chars().enumerate() {
            self.draw(x + i as i32, y, c);
        }
    }

    pub fn draw_border(&mut self, x: i32, y: i32, w: i32, h: i32) {
        const TL: char = 0xDA as char;
        const TR: char = 0xBF as char;
        const BL: char = 0xC0 as char;
        const BR: char = 0xD9 as char;
        const VT: char = 0xB3 as char;
        const HT: char = 0xC4 as char;

        let x0 = x;
        let x1 = x + w - 1;
        let y0 = y;
        let y1 = y + h - 1;

        self.draw(x0, y0, TL);
        self.draw(x1, y0, TR);
        self.draw(x1, y1, BR);
        self.draw(x0, y1, BL);

        for x in x0 + 1..=x1 - 1 {
            self.draw(x, y0, HT);
            self.draw(x, y1, HT);
        }

        for y in y0 + 1..=y1 - 1 {
            self.draw(x0, y, VT);
            self.draw(x1, y, VT);
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let x0 = x;
        let x1 = x + w - 1;
        let y0 = y;
        let y1 = y + h - 1;

        for y in y0..=y1 {
            for x in x0..=x1 {
                self.draw(x, y, ' ');
            }
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct ButtonState {
    pub just_down: bool,
    pub just_up: bool,
    pub held: bool,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct KeyState {
    pub just_down: bool,
    pub just_up: bool,
    pub held: bool,
}
