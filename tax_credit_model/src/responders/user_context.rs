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

    pub fn set_user(&mut self, user: User) {
        self.user = Some(user);
    }

    pub fn is_logged_in(&self) -> bool {
        self.user.is_some()
    }

    pub fn is_logged_out(&self) -> bool {
        self.user.is_none()
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
                Error::unknown("No user client found"),
            ));
        }

        let user_client = user_client_outcome.unwrap();
        let user_id = user_id_cookie.value().parse::<UserId>();

        if user_id.is_err() {
            return Outcome::Failure((
                Status::BadRequest,
                Error::invalid_argument("User id cookie is not a valid user id"),
            ));
        }

        user_client.get_user_by_id(&user_id.unwrap()).map_or_else(
            |err| Outcome::Failure((Status::Unauthorized, err)),
            |user| Outcome::Success(UserContext { user: Some(user) }),
        )
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user_context_outcome = UserContext::from_request(request).await;

        match user_context_outcome {
            Outcome::Success(user_context) => {
                if user_context.is_logged_in() {
                    Outcome::Success(user_context.user.unwrap())
                } else {
                    Outcome::Failure((
                        Status::Unauthorized,
                        Error::unauthenticated("User is not authenticated"),
                    ))
                }
            }
            Outcome::Failure(reason) => Outcome::Failure(reason),
            Outcome::Forward(reason) => Outcome::Forward(reason),
        }
    }
}
