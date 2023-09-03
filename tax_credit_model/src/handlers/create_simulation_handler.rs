use askama::Template;
use rocket::{get, response::content::RawHtml};

#[derive(Template)]
#[template(path = "simulation.html")]
struct CreateSimulationTemplate;

#[get("/")]
pub fn create_simulation_handler() -> RawHtml<String> {
    RawHtml(CreateSimulationTemplate {}.to_string())
}
