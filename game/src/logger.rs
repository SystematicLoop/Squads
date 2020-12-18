use cherry::graphics::colour::Colour;
use std::collections::VecDeque;

pub mod colour {
    use cherry::graphics::colour::Colour;

    pub const TEXT: Colour = Colour::DARK_GRAY;
    pub const NAME: Colour = Colour::GRAY;
    pub const HEALTH: Colour = Colour::DARK_RED;
    pub const ARMOUR: Colour = Colour::DARK_GREEN;
    pub const SHIELD: Colour = Colour::DARK_BLUE;
}

#[derive(Debug, Default)]
pub struct Logger {
    pub messages: VecDeque<Message>,
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message(&mut self) -> MessageBuilder {
        MessageBuilder::new(self)
    }
}

#[derive(Debug, Default)]
pub struct Message {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Token {
    pub content: String,
    pub colour: Colour,
}

#[derive(Debug)]
pub struct MessageBuilder<'a> {
    logger: &'a mut Logger,
    tokens: Vec<Token>,
}

impl<'a> MessageBuilder<'a> {
    fn new(logger: &'a mut Logger) -> Self {
        Self {
            logger,
            tokens: Vec::new(),
        }
    }

    pub fn with(mut self, content: &str, colour: Colour) -> Self {
        let token = Token {
            content: String::from(content),
            colour,
        };

        self.tokens.push(token);
        self
    }

    pub fn build(self) {
        let message = Message {
            tokens: self.tokens,
        };

        self.logger.messages.push_back(message);
    }
}
