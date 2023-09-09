use rocket::fs::relative;
use tax_credit_model_server::{
    jobs::ercot_data_retriever::ErcotDataRetrieverJob,
    server::{init_service, ServerConfiguration},
};

#[macro_use]
extern crate rocket;

#[launch]
pub async fn rocket() -> _ {
    let assets_directory =
        std::env::var("ASSETS_DIRECTORY").unwrap_or_else(|_| relative!("assets").to_string());
    let data_directory = std::env::var("DATA_DIRECTORY")
        .unwrap_or_else(|_| "/Users/evancoulson/hydrogen-trading/data".to_string());
    let configuration = ServerConfiguration::new(&assets_directory);

    tokio::task::spawn(async move {
        ErcotDataRetrieverJob::new(&data_directory).run();
    });

    init_service(configuration)
}
