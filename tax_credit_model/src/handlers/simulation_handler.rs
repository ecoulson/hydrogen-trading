use rocket::{get, http::Status, State};

use crate::{
    components::{
        electrolyzer::{ElectrolyzerDetails, ElectrolyzerSelector},
        page::{Page, PageResponse},
        simulation::SimulationView,
    },
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient,
        simulation_selection::SimulationSelectionClient, user::UserClient,
    },
    responders::{htmx_responder::HtmxHeadersBuilder, user_context::UserContext},
    schema::{
        simulation_schema::{SimulationId, SimulationPage},
        time::DateTimeRange,
        user::User,
    },
};

#[get("/simulation/<simulation_id>")]
pub fn simulation_handler(
    user_context: UserContext,
    simulation_id: SimulationId,
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
            electrolyzer_details: ElectrolyzerDetails::render_selected(electrolyzer),
            simulation_view: SimulationView::render(
                DateTimeRange {
                    start: String::from("2023-01-01T00:00"),
                    end: String::from("2023-07-31T23:59"),
                },
                ElectrolyzerSelector::render(electrolyzer_id, electrolyzers),
            ),
        },
    )
}
