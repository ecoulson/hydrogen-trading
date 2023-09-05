use tax_credit_model_server::server::{init_service, ServerConfiguration};

#[macro_use]
extern crate rocket;

#[launch]
pub fn rocket() -> _ {
    let configuration = ServerConfiguration::new();

    init_service(configuration)
}
