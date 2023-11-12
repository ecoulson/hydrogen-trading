use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};

use crate::schema::errors::Error;

#[derive(Debug)]
pub struct Context {
    simulation_id: i32,
    location: String
}

impl Context {
    pub fn simulation_id(&self) -> i32 {
        self.simulation_id
    }

    pub fn location(&self) -> &str {
        &self.location
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Context {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(current_url) = request.headers().get_one("Hx-Current-Url") {
            return Outcome::Success(Context { simulation_id: 0, location: String::from(current_url) });
        }

        Outcome::Failure((
            Status::NotFound,
            Error::create_not_found_error("No current url provided"),
        ))
    }
}
