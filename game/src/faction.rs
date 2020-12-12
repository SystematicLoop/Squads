use crate::gid::Gid;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Faction {
    pub name: String,
    pub units: HashSet<Gid>,
}

#[derive(Debug, Deserialize)]
pub struct FactionDef {
    pub name: String,
}
