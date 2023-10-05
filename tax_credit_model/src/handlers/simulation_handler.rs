use askama::Template;
use rocket::{get, State};

use crate::{
    persistance::electrolyzer::ElectrolyzerClient,
    responders::htmx_responder::HtmxTemplate,
    schema::{errors::BannerError, time::DateTimeRange},
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_form_template::SimulationFormTemplate,
    },
};

#[derive(Template)]
#[template(path = "pages/simulation.html")]
pub struct CreateSimulationTemplate {
    simulation_form: SimulationFormTemplate,
}

#[get("/")]
pub fn simulation_handler(
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<CreateSimulationTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(|err| BannerError {
            message: err.to_string(),
        })?;

    Ok(CreateSimulationTemplate {
        simulation_form: SimulationFormTemplate {
            generation_range: DateTimeRange {
                start: String::from("2023-01-01T00:00"),
                end: String::from("2023-07-31T23:59"),
            },
            electrolyzer_selector: ElectrolyzerSelectorTemplate { electrolyzers },
        },
    }
    .into())
}
