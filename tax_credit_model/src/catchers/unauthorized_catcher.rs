use askama::Template;
use rocket::{catch, Request};

use crate::responders::htmx_responder::{HtmxHeadersBuilder, HtmxTemplate};

#[derive(Debug, Template)]
#[template(path = "pages/unauthorized.html")]
pub struct UnauthorizedPage;

#[catch(401)]
pub fn unauthorized_catcher(request: &Request) -> HtmxTemplate<UnauthorizedPage> {
    if let Some(user_id) = request.cookies().get("user_id") {
        HtmxTemplate::new(
            HtmxHeadersBuilder::new()
                .set_cookie(&format!("user_id={}; Max-Age=-1", user_id))
                .build(),
            UnauthorizedPage {},
        )
    } else {
        HtmxTemplate::template(UnauthorizedPage {})
    }
}
