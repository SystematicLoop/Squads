use iced::{
    button,
    pick_list,
    Application,
    Button,
    Column,
    Command,
    Element,
    PickList,
    Settings,
    Text,
};

struct Editor {
    hello_button: button::State,
    pick_list: pick_list::State<ObjectKind>,
    selected_object_kind: Option<ObjectKind>,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    None,
    SelectObjectKind(ObjectKind),
}

impl Application for Editor {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                hello_button: button::State::default(),
                pick_list: pick_list::State::default(),
                selected_object_kind: Some(ObjectKind::Unit),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Squads Editor")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::None => {}
            Message::SelectObjectKind(kind) => {
                self.selected_object_kind = Some(kind);
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let pick_list = PickList::new(
            &mut self.pick_list,
            &ObjectKind::ALL[..],
            self.selected_object_kind,
            |kind| Message::SelectObjectKind(kind),
        );

        Column::new()
            .padding(10)
            .push(
                Button::new(&mut self.hello_button, Text::new("Hello, world!"))
                    .on_press(Message::None),
            )
            .push(pick_list)
            .into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ObjectKind {
    Unit,
    Weapon,
}

impl std::fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ObjectKind::Unit => "Unit",
                ObjectKind::Weapon => "Weapon",
            }
        )
    }
}

impl ObjectKind {
    const ALL: [ObjectKind; 2] = [ObjectKind::Unit, ObjectKind::Weapon];
}

fn main() {
    Editor::run(Settings::default()).unwrap();
}
