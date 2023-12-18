use rocket::{get, State};

use crate::{
    components::{
        electrolyzer::ElectrolyzerList,
        page::{Page, PageResponse},
        simulation::SimulationList,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient, user::UserClient,
    },
    responders::{htmx_responder::HtmxHeadersBuilder, user_context::UserContext},
    schema::{index::IndexResponse, user::User},
};

#[get("/")]
pub fn index_handler(
    user_context: UserContext,
    user_client: &State<Box<dyn UserClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> PageResponse<IndexResponse> {
    let mut cookie = None;

    if user_context.is_logged_out() {
        let user = user_client.create_user(&User::default())?;
        cookie = Some(format!("user_id={}", user.id()));
    }

    Page::page(
        HtmxHeadersBuilder::new().set_cookie_if(cookie).build(),
        IndexResponse {
            electrolyzer_list: ElectrolyzerList::render(electrolyzer_client.list_electrolyzers()?),
            simulation_list: SimulationList::render(simulation_client.list_simulations()?),
        },
    )
}
