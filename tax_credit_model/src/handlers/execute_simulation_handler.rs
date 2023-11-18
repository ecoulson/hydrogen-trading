use rocket::{form::Form, post, State};

use crate::{
    logic::simulation::{simulate, SimulationState},
    persistance::{
        electrolyzer::ElectrolyzerClient, grid::GridClient, simulation::SimulationClient,
    },
    responders::{
        context::Context,
        htmx_responder::{HtmxHeadersBuilder, HtmxTemplate},
    },
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
    context: Context,
    request: Form<ExecuteSimulationRequest>,
    power_grid_fetcher: &State<Box<dyn GridClient>>,
    electrolyzer_client: &State<Box<dyn ElectrolyzerClient>>,
    simulation_client: &State<Box<dyn SimulationClient>>,
) -> Result<HtmxTemplate<ExecuteSimulationResponse>, HtmxTemplate<BannerError>> {
    let mut context = context;
    let electrolyzer = electrolyzer_client
        .get_electrolyzer(request.electrolyzer_id)
        .map_err(BannerError::create_from_error)?;
    let power_grid = power_grid_fetcher
        .get_power_grid()
        .map_err(BannerError::create_from_error)?;
    let mut next_simulation = SimulationState::default();
    next_simulation.electrolyzer_id = electrolyzer.id;
    dbg!("bongo");
    let next_simulation = simulation_client
        .create_simulation_state(&next_simulation)
        .map_err(BannerError::create_from_error)?;
    let next_url = &format!("{}", next_simulation.id);
    let location = context.mut_location();
    location.set_path(&next_url);

    Ok(HtmxTemplate::new(
        HtmxHeadersBuilder::new()
            .replace_url(&location.build_url())
            .build(),
        ExecuteSimulationResponse {
            simulation_form: SimulationFormTemplate {
                generation_range: DateTimeRange::default(),
                electrolyzer_selector: ElectrolyzerSelectorTemplate {
                    electrolyzers: electrolyzer_client
                        .list_electrolyzers()
                        .map_err(BannerError::create_from_error)?,
                    selected_id: electrolyzer.id,
                    simulation_id: next_simulation.id,
                },
                simulation_id: next_simulation.id,
            },
            simulation_result: simulate(
                request.simulation_id,
                &power_grid,
                &electrolyzer,
                &request.simulation_time_range,
                simulation_client.inner(),
            )
            .map_err(BannerError::create_from_error)?,
        },
    ))
}
