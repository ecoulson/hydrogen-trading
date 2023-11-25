use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub enum ClientEvent {
    #[default]
    CreateElectrolyzer,
    SelectSimulation,
    SelectElectrolyzer,
    ListSimulations,
}

impl Display for ClientEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::SelectSimulation => write!(f, "select-simulation"),
            Self::SelectElectrolyzer => write!(f, "select-electrolyzer"),
            Self::CreateElectrolyzer => write!(f, "create-electrolyzer"),
            Self::ListSimulations => write!(f, "list-simulations"),
        }
    }
}
