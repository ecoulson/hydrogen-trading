use askama::Template;
use rocket::{get, http::Status, State};

use crate::{
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient, user::UserClient,
    },
    responders::{
        htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
        user_context::UserContext,
    },
    schema::user::User,
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
) -> Result<HtmxTemplate<IndexResponse>, Status> {
    let mut cookie = None;

    if user_context.user().is_none() {
        let user = user_client
            .create_user(&User::default())
            .map_err(|_| Status::InternalServerError)?;

        cookie = Some(format!("user_id={}", user.id()))
    }

    let simulations = simulation_client
        .list_simulations()
        .map_err(|_| Status::InternalServerError)?;
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(|_| Status::InternalServerError)?;

    Ok(HtmxTemplate::new(
        HtmxHeadersBuilder::new().set_cookie_if(cookie).build(),
        IndexResponse {
            electrolyzer_list: ListElectrolyzersTemplate {
                search_results: ElectrolyzerSearchResults {
                    electrolyzers,
                    selected_id: None,
                },
            },
            simulation_list: ListSimulationResponse { simulations },
        },
    ))
}
