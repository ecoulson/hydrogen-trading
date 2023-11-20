use askama::Template;
use rocket::{get, State};

use crate::{
    logic::simulation::SimulationState,
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient, user::UserClient,
    },
    responders::{
        htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
        user_context::UserContext,
    },
    schema::{errors::BannerError, user::User},
    templates::list_electrolyzers_template::{
        ElectrolyzerSearchResults, ListElectrolyzersTemplate,
    },
};

use super::list_simulation_handler::ListSimulationResponse;

#[derive(Template, Debug)]
#[template(path = "pages/index.html")]
pub struct IndexResponse {
    electrolyzer_list: ListElectrolyzersTemplate,
    simulation_list: ListSimulationResponse,
}

#[get("/")]
pub fn index_handler(
    user_context: UserContext,
    user_client: &State<Box<dyn UserClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<IndexResponse>, HtmxTemplate<BannerError>> {
    let mut cookie = None;

    if user_context.user().is_none() {
        let user = user_client
            .create_user(&User::default())
            .map_err(BannerError::create_from_error)?;

        cookie = Some(format!("user_id={}", user.id()))
    }

    let mut simulations = simulation_client
        .list_simulations()
        .map_err(BannerError::create_from_error)?;
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;

    if simulations.is_empty() {
        simulations.push(
            simulation_client
                .create_simulation_state(&SimulationState::default())
                .map_err(BannerError::create_from_error)?,
        );
    }

    Ok(HtmxTemplate::new(
        HtmxHeadersBuilder::new().set_cookie_if(cookie).build(),
        IndexResponse {
            electrolyzer_list: ListElectrolyzersTemplate {
                search_results: ElectrolyzerSearchResults { electrolyzers },
            },
            simulation_list: ListSimulationResponse { simulations },
        },
    ))
}
