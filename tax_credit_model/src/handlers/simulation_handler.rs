use askama::Template;
use rocket::get;

use crate::responders::htmx_responder::HtmxTemplate;

#[derive(Template)]
#[template(path = "pages/simulation.html")]
pub struct CreateSimulationTemplate;

#[get("/")]
pub fn simulation_handler() -> HtmxTemplate<CreateSimulationTemplate> {
    CreateSimulationTemplate {}.into()
}
