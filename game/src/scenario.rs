use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Scenario {
    pub starting_faction: String,
}
