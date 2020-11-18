#[derive(Debug, Copy, Clone)]
pub enum Button {
    Left,
    Right,
    Middle,

    // Ensure this is the last item
    // in the list.
    Unknown,
}

impl Button {
    pub const fn count() -> usize {
        Self::Unknown as usize
    }
}
