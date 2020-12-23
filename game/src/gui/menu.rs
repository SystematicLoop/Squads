use cherry::{
    engine::Cherry,
    graphics::colour::Colour,
};

#[derive(Debug)]
pub struct Menu<T> {
    title: String,
    items: Vec<Item<T>>,
}

impl<T> Menu<T> {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, text: &str, data: T) {
        self.items.push(Item {
            text: String::from(text),
            data,
        })
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn items(&self) -> &[Item<T>] {
        &self.items
    }

    pub fn get(&self, index: usize) -> Option<&Item<T>> {
        self.items.get(index)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[derive(Debug)]
pub struct Item<T> {
    text: String,
    data: T,
}

impl<T> Item<T> {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

pub fn draw_menu<T>(
    engine: &mut Cherry,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    menu: &Menu<T>,
    selection: usize,
) {
    // Cache previous terminal properties.
    let prev_fg = engine.get_fg();
    let prev_bg = engine.get_bg();

    // Setup
    engine.fill_rect(x, y, w, h);

    // Title
    engine.set_fg(Colour::DARK_CYAN);
    engine.draw_str(x, y, menu.title());

    // Separator
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.draw_h_line(x, y + 1, w, 0xC4 as char);

    // Menu items
    for (i, item) in menu.items().iter().enumerate() {
        if i == selection {
            engine.set_fg(Colour::BLACK);
            engine.set_bg(Colour::WHITE);
            engine.draw_str(
                x,
                y + 2 + i as i32,
                &format!("> {:width$}", item.text(), width = w as usize - 2),
            );
        } else {
            engine.set_fg(Colour::WHITE);
            engine.set_bg(Colour::BLACK);
            engine.draw_str(
                x,
                y + 2 + i as i32,
                &format!("  {:width$}", item.text(), width = w as usize - 2),
            );
        }
    }

    // Separator
    engine.set_fg(Colour::VERY_DARK_GRAY);
    engine.set_bg(Colour::BLACK);
    engine.draw_h_line(x, h - 1, w, 0xC4 as char);

    // Restore terminal properties.
    engine.set_fg(prev_fg);
    engine.set_bg(prev_bg);
}
