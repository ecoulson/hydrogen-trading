use askama::Template;
use rocket::{get, State};
use serde::{Deserialize, Serialize};

use crate::{
    logic::simulation::SimulationState,
    persistance::{electrolyzer::ElectrolyzerClient, simulation::SimulationClient},
    responders::htmx_responder::HtmxTemplate,
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError, time::DateTimeRange},
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_form_template::SimulationFormTemplate,
    },
};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "pages/simulation.html")]
pub struct SimulationPage {
    initial_state: String,
    simulation_form: Option<SimulationFormTemplate>,
    electrolyzer_details: Option<ElectrolyzerDetailsTemplate>,
}

#[get("/?<simulation_id>")]
pub fn simulation_handler(
    simulation_id: Option<i32>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<SimulationPage>, HtmxTemplate<BannerError>> {
    let simulation_state = match simulation_id {
        None => simulation_client
            .create_simulation_state(&SimulationState::default())
            .map_err(BannerError::create_from_error)?,
        Some(id) => simulation_client
            .ensure_simulation_exists(&id)
            .map_err(BannerError::create_from_error)?,
    };
    let url_state = format!("?simulation_id={}", simulation_state.id);
    let electrolyzer_id = simulation_state.electrolyzer.id.clone();
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;

    if electrolyzers.is_empty() {
        return Ok(SimulationPage {
            simulation_form: None,
            electrolyzer_details: None,
            initial_state: url_state,
        }
        .into());
    }

    Ok(SimulationPage {
        initial_state: url_state,
        electrolyzer_details: Some(ElectrolyzerDetailsTemplate {
            electrolyzer: simulation_state.electrolyzer,
            selected: true,
        }),
        simulation_form: Some(SimulationFormTemplate {
            generation_range: DateTimeRange {
                start: String::from("2023-01-01T00:00"),
                end: String::from("2023-07-31T23:59"),
            },
            electrolyzer_selector: ElectrolyzerSelectorTemplate {
                electrolyzers,
                selected_id: electrolyzer_id,
            },
        }),
    }
    .into())
}
