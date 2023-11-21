use askama::Template;
use rocket::{get, State};
use serde::{Deserialize, Serialize};

use crate::{
    persistance::{
        electrolyzer::ElectrolyzerClient, simulation::SimulationClient, user::UserClient,
    },
    responders::{
        htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
        user_context::UserContext,
    },
    schema::{electrolyzer::ElectrolyzerDetailsTemplate, errors::BannerError, time::DateTimeRange, user::User},
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_form_template::SimulationFormTemplate,
    },
};

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq)]
#[template(path = "pages/simulation.html")]
pub struct SimulationPage {
    simulation_id: i32,
    simulation_form: SimulationFormTemplate,
    electrolyzer_details: ElectrolyzerDetailsTemplate,
}

#[get("/simulation/<simulation_id>")]
pub fn simulation_handler(
    user_context: UserContext,
    simulation_id: i32,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
    user_client: &State<Box<dyn UserClient>>,
) -> Result<HtmxTemplate<SimulationPage>, HtmxTemplate<BannerError>> {
    let mut cookie = None;

    if user_context.user().is_none() {
        let user = user_client
            .create_user(&User::default())
            .map_err(BannerError::create_from_error)?;

        cookie = Some(format!("user_id={}", user.id()))
    }

    let mut user_context = user_context;
    let user = user_context
        .user_mut()
        .ok_or_else(|| BannerError::create_from_message("User not logged in"))?;
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(BannerError::create_from_error)?;
    let simulation_state = simulation_client
        .get_simulation_state(&simulation_id)
        .map_err(BannerError::create_from_error)?;
    user.set_simulation_id(simulation_state.id);
    user_client
        .update_user(&user)
        .map_err(BannerError::create_from_error)?;
    let electrolyzer = electrolyzers
        .iter()
        .find(|electrolyzer| electrolyzer.id == simulation_state.electrolyzer_id)
        .map(|electrolyzer| electrolyzer.clone())
        .ok_or_else(|| BannerError::create_from_message("Could not find electrolyzer"))?;
    let electrolyzer_id = electrolyzer.id.clone();

    Ok(HtmxTemplate::new(
        HtmxHeadersBuilder::new()
        .set_cookie_if(cookie)
        .build(),
        SimulationPage {
            simulation_id: simulation_state.id,
            electrolyzer_details: ElectrolyzerDetailsTemplate {
                electrolyzer,
                selected: true,
            },
            simulation_form: SimulationFormTemplate {
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
    ))
}
