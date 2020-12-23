use crate::gid::Gid;

use petgraph::{
    graphmap::GraphMap,
    Directed,
};

use serde::Deserialize;

use std::collections::HashSet;

pub type World = GraphMap<Gid, Edge, Directed>;

#[derive(Debug, Deserialize)]
pub struct AreaDef {
    pub name: String,
}

#[derive(Debug)]
pub struct Area {
    pub name: String,
    pub units: HashSet<Gid>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge;
