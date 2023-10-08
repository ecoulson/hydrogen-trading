use askama::Template;
use rocket::get;
use serde::{Deserialize, Serialize};

use crate::responders::htmx_responder::HtmxTemplate;

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq, Clone, Copy)]
#[template(path = "pages/create_electrolyzer.html")]
pub struct CreateElectrolyzerPage {}

#[get("/create_electrolyzer")]
pub fn create_electrolyzer_page_handler() -> HtmxTemplate<CreateElectrolyzerPage> {
    CreateElectrolyzerPage {}.into()
}
