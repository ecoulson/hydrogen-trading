use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};

use crate::schema::errors::Error;

#[derive(Debug, Default)]
pub struct ClientContext<'r> {
    location: Url<'r>,
}

impl<'r> ClientContext<'r> {
    pub fn location(&self) -> &'r Url {
        &self.location
    }

    pub fn mut_location(&mut self) -> &'r mut Url {
        &mut self.location
    }
}

pub enum UrlParseState {
    Scheme,
    Authority,
    Path,
    Parameters,
}

#[derive(Debug, Default)]
pub struct Url<'r> {
    scheme: &'r str,
    authority: &'r str,
    path: &'r str,
    parameters: &'r str,
}

impl<'r> Url<'r> {
    pub fn set_path(&mut self, path: &'r str) {
        self.path = path;
    }

    pub fn build_url(&self) -> String {
        format!(
            "{}://{}/{}{}",
            self.scheme, self.authority, self.path, self.parameters
        )
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientContext<'r> {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(current_url) = request.headers().get_one("Hx-Current-Url") {
            let mut location = Url::default();
            let mut start = 0;
            let mut state = UrlParseState::Scheme;
            let url_iterator = current_url.chars().enumerate();

            for (i, ch) in url_iterator {
                match state {
                    UrlParseState::Scheme => {
                        if ch == ':' {
                            location.scheme = &current_url[start..i];
                            state = UrlParseState::Authority;
                            start = i + 3;
                        }
                    }
                    UrlParseState::Authority => {
                        if ch == '/' && start <= i {
                            location.authority = &current_url[start..i];
                            state = UrlParseState::Path;
                            start = i;
                        }
                    }
                    UrlParseState::Path => {
                        if ch == '?' {
                            location.path = &current_url[start..i];
                            state = UrlParseState::Parameters;
                            start = i;
                        }
                    }
                    UrlParseState::Parameters => break,
                }
            }

            match state {
                UrlParseState::Path => location.path = &current_url[start..current_url.len()],
                UrlParseState::Parameters => {
                    location.parameters = &current_url[start..current_url.len()]
                }
                _ => (),
            }

            return Outcome::Success(ClientContext { location });
        }

        Outcome::Failure((
            Status::NotFound,
            Error::create_not_found_error("No client context"),
        ))
    }
}
