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
    pub area: String,
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
    pub area_id: Gid,
    pub weapon_id: Gid,

    pub health: Stat,
    pub armour: Stat,
    pub shield: Stat,
    pub stamina: Stat,

    pub accuracy: f32,
    pub speed: u16,
}

#[derive(Debug, Default, Copy, Clone, Deserialize)]
pub struct Stat {
    val: u16,
    max: u16,
    change: i16,
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

    pub fn val(&self) -> u16 {
        self.val
    }

    pub fn max(&self) -> u16 {
        self.max
    }

    pub fn change(&self) -> i16 {
        self.change
    }

    pub fn change_by(&mut self, delta: i16) {
        if delta < 0 {
            self.val = self.val.saturating_sub((-delta) as u16);
        } else if delta > 0 {
            self.val = self.val.saturating_add(delta as u16).min(self.max);
        }

        if self.change ^ delta < 0 {
            self.change = delta;
        } else {
            self.change += delta;
        }
    }

    pub fn clear_change(&mut self) {
        self.change = 0;
    }

    pub fn full(&self) -> bool {
        self.val == self.max
    }
}
