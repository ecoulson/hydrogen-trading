use askama::Template;
use rocket::catch;

use crate::responders::htmx_responder::HtmxTemplate;

#[derive(Debug, Template)]
#[template(path = "pages/not_found.html")]
pub struct NotFoundPage;

#[catch(404)]
pub fn not_found_catcher() -> HtmxTemplate<NotFoundPage> {
    NotFoundPage {}.into()
}
