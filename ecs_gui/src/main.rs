use cherry::{
    engine::Cherry,
    graphics::colour::Colour,
    input::key::Key,
    CherryApp,
};

use hecs::*;

struct Application {
    world: World,
    interactable: Vec<Entity>,
    interactable_index: usize,
}

impl Application {
    fn render_gui(&mut self, engine: &mut Cherry) {
        let mut query = self.world.query::<(&ElementKind, &Position, &Hierarchy)>();

        for (entity, (kind, position, hierarchy)) in query.iter() {
            let is_interacting = self.interactable[self.interactable_index] == entity;
            let mut offset = Position { x: 0, y: 0 };
            if let Some(parent) = hierarchy.parent {
                let position = self.world.get::<Position>(parent).unwrap();
                offset.x += position.x;
                offset.y += position.y;
            }

            match kind {
                ElementKind::Label => {
                    let label = self.world.get::<Label>(entity).unwrap();
                    let fg = engine.get_fg();
                    let bg = engine.get_bg();
                    if is_interacting {
                        engine.set_fg(bg);
                        engine.set_bg(fg);
                    }

                    engine.draw_str(position.x + offset.x, position.y + offset.y, &label.text);
                    engine.set_fg(fg);
                    engine.set_bg(bg);
                }
                ElementKind::Container => {
                    let size = self.world.get::<Size>(entity).unwrap();
                    let bg = engine.get_bg();
                    engine.set_bg(Colour::VERY_DARK_BLUE);
                    engine.fill_rect(position.x, position.y, size.w, size.h);
                    engine.set_bg(bg);
                }
            }
        }
    }
}

impl CherryApp for Application {
    fn on_update(&mut self, engine: &mut Cherry) {
        engine.set_fg(Colour::WHITE);
        engine.set_bg(Colour::BLACK);
        engine.clear();

        if engine.key(Key::Down).just_down {
            self.interactable_index = (self.interactable_index + 1) % self.interactable.len();
        }

        self.render_gui(engine);
    }
}

fn gui_label(world: &mut World, x: i32, y: i32, text: &str) -> Entity {
    // Labels have text. If they have a size to represent an accurate size, then
    // the text could be changed and the size would have to be updated.
    let label = Label {
        text: String::from(text),
    };

    let position = Position { x, y };

    let hierarchy = Hierarchy {
        parent: None,
        children: Vec::new(),
    };

    let entity = world.spawn((ElementKind::Label, label, hierarchy, position));
    entity
}

fn gui_container(world: &mut World, x: i32, y: i32, w: i32, h: i32) -> Entity {
    let position = Position { x, y };

    let size = Size { w, h };

    let hierarchy = Hierarchy {
        parent: None,
        children: Vec::new(),
    };

    let entity = world.spawn((ElementKind::Container, hierarchy, position, size));
    entity
}

fn gui_add_child(world: &mut World, parent: Entity, child: Entity) {
    // Assumes the child isn't already among the parent's children.
    let mut hierarchy = world.get_mut::<Hierarchy>(parent).unwrap();
    hierarchy.children.push(child);

    // Ignoring the case where the child already has a parent.
    let mut hierarchy = world.get_mut::<Hierarchy>(child).unwrap();
    hierarchy.parent = Some(parent);
}

enum ElementKind {
    Label,
    Container,
}

struct Position {
    x: i32,
    y: i32,
}

struct Size {
    w: i32,
    h: i32,
}

struct Hierarchy {
    parent: Option<Entity>,
    children: Vec<Entity>,
}

struct Label {
    text: String,
}

struct Interactable;

fn main() {
    let mut world = World::default();

    // Todo: Add clipping functionality to engine.
    let container = gui_container(&mut world, 10, 10, 5, 5);
    let label_alpha = gui_label(&mut world, 0, 0, "Alpha");
    let label_beta = gui_label(&mut world, 0, 2, "Beta");
    let label_gamma = gui_label(&mut world, 2, 4, "Gamma");

    gui_add_child(&mut world, container, label_gamma);

    let mut app = Application {
        world,
        interactable: Vec::new(),
        interactable_index: 0,
    };

    app.interactable.push(label_alpha);
    app.interactable.push(label_beta);
    app.interactable.push(label_gamma);

    let mut engine = Cherry::new("ECS Interface", 40, 30, "res/fonts/cp437_16x16.png");
    engine.run(&mut app);
}
