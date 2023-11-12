use rocket::{form::Form, post, State};

use crate::{
    logic::simulation::simulate,
    persistance::{
        electrolyzer::ElectrolyzerClient, grid::GridClient, simulation::SimulationClient,
    },
    responders::htmx_responder::HtmxTemplate,
    schema::{
        errors::BannerError,
        simulation_schema::{ExecuteSimulationRequest, ExecuteSimulationResponse},
        time::DateTimeRange,
    },
    templates::{
        list_electrolyzers_template::ElectrolyzerSelectorTemplate,
        simulation_form_template::SimulationFormTemplate,
    },
};

#[post("/execute_simulation", data = "<request>")]
pub fn execute_simulation(
    request: Form<ExecuteSimulationRequest>,
    power_grid_fetcher: &State<Box<dyn GridClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ExecuteSimulationResponse>, HtmxTemplate<BannerError>> {
    let electrolyzer = electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .map_err(BannerError::create_from_error)?;
    let power_grid = power_grid_fetcher
        .get_power_grid()
        .map_err(BannerError::create_from_error)?;

    Ok(ExecuteSimulationResponse {
        simulation_form: SimulationFormTemplate {
            generation_range: DateTimeRange::default(),
            electrolyzer_selector: ElectrolyzerSelectorTemplate {
                electrolyzers: electrolyzer_client
                    .list_electrolyzers()
                    .map_err(BannerError::create_from_error)?,
                selected_id: electrolyzer.id,
            },
        },
        simulation_result: simulate(
            0,
            &power_grid,
            &electrolyzer,
            &request.simulation_time_range,
            simulation_client.inner(),
        )
        .map_err(BannerError::create_from_error)?,
    }
    .into())
}
