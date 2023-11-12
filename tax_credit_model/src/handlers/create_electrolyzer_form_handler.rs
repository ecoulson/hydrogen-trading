use askama::Template;
use rocket::post;
use serde::{Deserialize, Serialize};

use crate::responders::htmx_responder::HtmxTemplate;

#[derive(Template, Deserialize, Serialize, Default, Debug, PartialEq, Clone, Copy)]
#[template(path = "components/create_electrolyzer.html")]
pub struct CreateElectrolyzerPage {}

#[post("/create_electrolyzer_form")]
pub fn create_electrolyzer_form_handler() -> HtmxTemplate<CreateElectrolyzerPage> {
    CreateElectrolyzerPage {}.into()
}
