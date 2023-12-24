use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Endpoint {
    #[default]
    None,
    IndexPage,
    FetchHydrogenProduction,
    FetchEnergyCosts,
    FetchHourlyHistogram,
    FetchEmissions,
    CreateElectrolyzer,
    SelectElectrolyzer,
    SelectSimulation,
    SearchElectrolyzers,
    CloseError,
    GetCreateElectrolyzerForm,
    ElectrolyzerSelector,
    ExecuteSimulation,
    GetElectrolyzer,
    GetSelectedElectrolyzer,
    GetSelectedSimulation,
    InitializeSimulation,
    ListElectrolyzers,
    ListSimulations,
    SimulationPage,
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetElectrolyzer => write!(f, "/get_electrolyzer"),
            Self::CloseError => write!(f, "/close_error"),
            Self::ListElectrolyzers => write!(f, "/list_electrolyzers"),
            Self::SelectElectrolyzer => write!(f, "/select_electrolyzer"),
            Self::SelectSimulation => write!(f, "/select_simulation"),
            Self::CreateElectrolyzer => write!(f, "/create_electrolyzer"),
            Self::GetSelectedElectrolyzer => write!(f, "/get_selected_electrolyzer"),
            Self::ElectrolyzerSelector => write!(f, "/electrolyzer_selector"),
            Self::GetCreateElectrolyzerForm => write!(f, "/create_electrolyzer_form"),
            Self::SearchElectrolyzers => write!(f, "/search_electrolyzers"),
            Self::InitializeSimulation => write!(f, "/initialize_simulation"),
            Self::GetSelectedSimulation => write!(f, "/get_selected_simulation"),
            Self::ListSimulations => write!(f, "/list_simulations"),
            Self::ExecuteSimulation => write!(f, "/execute_simulation"),
            _ => write!(f, ""),
        }
    }
}
