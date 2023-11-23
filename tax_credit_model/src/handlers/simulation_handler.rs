use askama::Template;
use rocket::{get, http::Status, State};
use serde::{Deserialize, Serialize};

use crate::{
    components::page::{Page, PageResponse},
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient, user::UserClient,
    },
    responders::{htmx_responder::HtmxHeadersBuilder, user_context::UserContext},
    schema::{
        electrolyzer::{
            ElectrolyzerDetailsActions, ElectrolyzerDetailsState, ElectrolyzerDetailsTemplate,
        },
        time::DateTimeRange,
        user::User,
    },
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate, simulation_view::SimulationView,
    },
};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "pages/simulation.html")]
pub struct SimulationPage {
    simulation_id: i32,
    simulation_view: SimulationView,
    electrolyzer_details: ElectrolyzerDetailsTemplate,
}

#[get("/simulation/<simulation_id>")]
pub fn simulation_handler(
    user_context: UserContext,
    simulation_id: i32,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    user_client: &State<Box<dyn UserClient>>,
    simulation_selection_client: &State<Box<dyn SimulationSelectionClient>>,
) -> PageResponse<SimulationPage> {
    let mut cookie = None;
    let mut user_context = user_context;

    if user_context.is_logged_out() {
        let user = user_client.create_user(&User::default())?;
        cookie = Some(format!("user_id={}", user.id()));
        user_context.set_user(user);
    }

    let user = user_context.user_mut().unwrap();
    let electrolyzers = electrolyzer_client.list_electrolyzers()?;
    let simulation_state = simulation_client.get_simulation_state(&simulation_id)?;
    let electrolyzer = electrolyzers
        .iter()
        .find(|electrolyzer| electrolyzer.id == simulation_state.electrolyzer_id)
        .ok_or_else(|| Status::NotFound)?
        .clone();
    let electrolyzer_id = electrolyzer.id.clone();
    simulation_selection_client.select(user.id().clone(), simulation_id)?;

    Page::page(
        HtmxHeadersBuilder::new().set_cookie_if(cookie).build(),
        SimulationPage {
            simulation_id: simulation_state.id,
            electrolyzer_details: ElectrolyzerDetailsTemplate {
                electrolyzer,
                state: ElectrolyzerDetailsState::Selected,
                actions: ElectrolyzerDetailsActions::Selectable,
            },
            simulation_view: SimulationView {
                generation_range: DateTimeRange {
                    start: String::from("2023-01-01T00:00"),
                    end: String::from("2023-07-31T23:59"),
                },
                electrolyzer_selector: ElectrolyzerSelectorTemplate {
                    electrolyzers,
                    selected_id: electrolyzer_id,
                },
            },
        },
    )
}
