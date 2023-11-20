use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request, State,
};

use crate::{
    persistance::user::UserClient,
    schema::{
        errors::Error,
        user::{User, UserId},
    },
};

#[derive(Debug)]
pub struct UserContext {
    user: Option<User>,
}

impl UserContext {
    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }

    pub fn user_mut(&mut self) -> Option<&mut User> {
        self.user.as_mut()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserContext {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user_id_cookie = request.cookies().get("user_id");

        if user_id_cookie.is_none() {
            return Outcome::Success(UserContext { user: None });
        }

        let user_id_cookie = user_id_cookie.unwrap();
        let user_client_outcome = request.guard::<&State<Box<dyn UserClient>>>().await;

        if user_client_outcome.is_failure() {
            return Outcome::Failure((
                Status::InternalServerError,
                Error::create_unknown_error("No user client found"),
            ));
        }

        let user_client = user_client_outcome.unwrap();
        let user_id = user_id_cookie.value().parse::<UserId>();

        if user_id.is_err() {
            return Outcome::Failure((
                Status::BadRequest,
                Error::create_invalid_argument_error("User id cookie is not a valid user id"),
            ));
        }

        user_client.get_user_by_id(&user_id.unwrap()).map_or_else(
            |err| Outcome::Failure((Status::NotFound, err)),
            |user| Outcome::Success(UserContext { user: Some(user) }),
        )
    }
}
