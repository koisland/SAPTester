use super::pet::SimplePet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Teams {
    pub friend_team: SimpleTeam,
    pub enemy_team: SimpleTeam,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleTeam {
    name: String,
    pets: Vec<Option<SimplePet>>,
}
