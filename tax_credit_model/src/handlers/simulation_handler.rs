use askama::Template;
use rocket::get;

use crate::{responders::htmx_responder::HtmxTemplate, schema::time::DateTimeRange};

#[derive(Template)]
#[template(path = "pages/simulation.html")]
pub struct CreateSimulationTemplate {
    generation_range: DateTimeRange,
}

#[get("/")]
pub fn simulation_handler() -> HtmxTemplate<CreateSimulationTemplate> {
    CreateSimulationTemplate {
        generation_range: DateTimeRange {
            start: String::from("2023-01-01T00:00"),
            end: String::from("2023-07-31T23:59"),
        },
    }
    .into()
}
