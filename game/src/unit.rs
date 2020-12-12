use crate::gid::Gid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UnitDef {
    pub name: String,

    pub health: Stat,
    pub armour: Stat,
    pub shield: Stat,

    pub stamina: u16,
    pub speed: u16,
}

#[derive(Debug, Deserialize)]
pub struct UnitSpawn {
    pub unit: String,
    pub role: String,
    pub faction: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub role: String,
    pub faction: Gid,

    pub health: Stat,
    pub armour: Stat,
    pub shield: Stat,

    pub stamina: u16,
    pub speed: u16,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Stat {
    pub val: u16,
    pub max: u16,
    pub resistance: Resistance,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Resistance {
    pub energy: f32,
    pub kinetic: f32,
    pub explosive: f32,
}
