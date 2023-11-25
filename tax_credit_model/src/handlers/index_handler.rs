use askama::Template;
use rocket::{get, State};

use crate::{
    client::events::ClientEvent,
    components::{
        button::{ButtonBuilder, ButtonVariant},
        event::EventListenerBuilder,
        page::{Page, PageResponse},
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient, user::UserClient,
    },
    responders::{htmx_responder::HtmxHeadersBuilder, user_context::UserContext},
    schema::user::User,
    templates::list_electrolyzers_template::{
        ElectrolyzerSearchResultsBuilder, ListElectrolyzersTemplate,
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
) -> PageResponse<IndexResponse> {
    let mut cookie = None;

    if user_context.is_logged_out() {
        let user = user_client.create_user(&User::default())?;
        cookie = Some(format!("user_id={}", user.id()));
    }

    Page::page(
        HtmxHeadersBuilder::new().set_cookie_if(cookie).build(),
        IndexResponse {
            electrolyzer_list: ListElectrolyzersTemplate {
                search_results: ElectrolyzerSearchResultsBuilder::new()
                    .electrolyzers(electrolyzer_client.list_electrolyzers()?)
                    .build(),
                list_simulation_listener: EventListenerBuilder::new()
                    .event(ClientEvent::ListSimulations)
                    .target("#sidebar")
                    .endpoint("/list_electrolyzers")
                    .build(),
                select_simulation_listener: EventListenerBuilder::new()
                    .event(ClientEvent::SelectSimulation)
                    .target("#sidebar")
                    .endpoint("/get_selected_electrolyzer")
                    .build(),
                create_electrolyzer_button: ButtonBuilder::new()
                    .variant(ButtonVariant::Primary)
                    .endpoint("/create_electrolyzer_form")
                    .target("#sidebar")
                    .text("Create Electrolyzer")
                    .build(),
            },
            simulation_list: ListSimulationResponse {
                simulations: simulation_client.list_simulations()?,
                create_electrolyzer_listener: EventListenerBuilder::new()
                    .event(ClientEvent::CreateElectrolyzer)
                    .endpoint("initialize_simulation")
                    .target("#dataplane")
                    .build(),
            },
        },
    )
}
