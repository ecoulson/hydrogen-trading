use rocket::{form::Form, post, FromForm, State};

use crate::{
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient, user::UserClient,
    },
    responders::{
        client_context::ClientContext,
        htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
        user_context::UserContext,
    },
    schema::{errors::BannerError, time::DateTimeRange},
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_form_template::SimulationFormTemplate,
    },
};

#[derive(Debug, FromForm)]
pub struct SelectSimulationRequest {
    pub simulation_id: i32,
}

#[post("/select_simulation", data = "<request>")]
pub fn select_simulation_handler(
    request: Form<SelectSimulationRequest>,
    user_context: UserContext,
    user_client: &State<Box<dyn UserClient>>,
    client_context: ClientContext,
    simulation_client: &State<Box<dyn SimulationClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<SimulationFormTemplate>, HtmxTemplate<BannerError>> {
    let mut user_context = user_context;
    let mut client_context = client_context;
    let user = user_context
        .user_mut()
        .ok_or_else(|| BannerError::create_from_message("User not logged in"))?;
    user.set_simulation_id(request.simulation_id);
    user_client
        .update_user(user)
        .map_err(BannerError::create_from_error)?;
    let simulation = simulation_client
        .get_simulation_state(&user.simulation_id())
        .map_err(BannerError::create_from_error)?;
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;
    let next_url = &format!("simulation/{}", simulation.id);
    let location = client_context.mut_location();
    location.set_path(&next_url);

    Ok(HtmxTemplate::new(
        HtmxHeadersBuilder::new()
            .replace_url(&location.build_url())
            .trigger("simulation-selected")
            .build(),
        SimulationFormTemplate {
            generation_range: DateTimeRange {
                start: String::from("2023-01-01T00:00"),
                end: String::from("2023-07-31T23:59"),
            },
            electrolyzer_selector: ElectrolyzerSelectorTemplate {
                electrolyzers,
                selected_id: simulation.electrolyzer_id,
            },
        },
    ))
}
