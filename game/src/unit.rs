#[derive(Debug)]
pub struct Unit {
    pub id: u32,
    pub name: String,
    pub faction: u8,
    pub health: u16,
    pub health_max: u16,
}
