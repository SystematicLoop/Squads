#[derive(Debug)]
pub struct Unit {
    pub id: u32,
    pub name: String,
    pub faction: u8,
    
    pub health: u16,
    pub health_max: u16,
    pub armour: u16,
    pub armour_max: u16,
    pub shield: u16,
    pub shield_max: u16,

    pub actions: u16,
    pub speed: u16,
}
