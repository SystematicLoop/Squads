use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeaponDef {
    pub name: String,
    pub role: String,
    pub ammo: u16,
    pub accuracy: f32,
    pub damage: Damage,
}

#[derive(Debug)]
pub struct Weapon {
    pub name: String,
    pub role: String,
    pub ammo: Ammo,
    pub accuracy: f32,
    pub damage: Damage,
}

#[derive(Debug, Copy, Clone)]
pub struct Ammo {
    pub val: u16,
    pub max: u16,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Damage {
    pub base: u16,
    pub pierce: u16,
    pub energy: u16,
    pub explosion: u16,
}
