use rocket::fs::relative;
use tax_credit_model_server::server::{init_service, ServerConfiguration};

#[macro_use]
extern crate rocket;

#[launch]
pub async fn rocket() -> _ {
    let assets_directory =
        std::env::var("ASSETS_DIRECTORY").unwrap_or_else(|_| relative!("assets").to_string());
    let configuration = ServerConfiguration::new(&assets_directory);

    init_service(configuration)
}
