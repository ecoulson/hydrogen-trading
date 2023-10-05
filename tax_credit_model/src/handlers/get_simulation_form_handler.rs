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

#[get("/simulation_form")]
pub fn get_simulation_form_handler(
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
) -> Result<HtmxTemplate<SimulationFormTemplate>, HtmxTemplate<BannerError>> {
    let electrolyzers = electrolyzer_client
        .list_electrolyzers()
        .map_err(|err| BannerError {
            message: err.to_string(),
        })?;

    Ok(SimulationFormTemplate {
        generation_range: DateTimeRange {
            start: String::from("2023-01-01T00:00"),
            end: String::from("2023-07-31T23:59"),
        },
        electrolyzer_selector: ElectrolyzerSelectorTemplate { electrolyzers },
    }
    .into())
}
