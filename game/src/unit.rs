use crate::gid::Gid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UnitDef {
    pub name: String,
    pub accuracy: f32,

    pub health: u16,
    pub armour: u16,
    pub shield: u16,
    pub stamina: u16,
    pub speed: u16,
}

#[derive(Debug, Deserialize)]
pub struct UnitSpawn {
    pub unit: String,
    pub role: String,
    pub faction: String,
    pub name: String,
    pub weapon: String,
}

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub role: String,

    pub faction_id: Gid,
    pub weapon_id: Gid,

    pub health: Stat,
    pub armour: Stat,
    pub shield: Stat,
    pub stamina: Stat,

    pub accuracy: f32,
    pub speed: u16,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Stat {
    pub val: u16,
    pub max: u16,
    pub change: i16,
}

impl Stat {
    pub fn new(max: u16) -> Self {
        Self {
            val: max,
            max,
            change: 0,
        }
    }

    pub fn ratio(&self) -> f32 {
        self.val as f32 / self.max as f32
    }
}
