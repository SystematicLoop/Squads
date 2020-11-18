#[derive(Debug, Copy, Clone)]
pub enum Key {
    Up,
    Down,
    Enter,
    Backspace,

    // Ensure this is the last item
    // in the list.
    Unknown,
}

impl Key {
    pub const fn count() -> usize {
        Self::Unknown as usize
    }
}
