use std::process;

use tax_credit_model_server::{
    jobs::ercot_data_retriever::ErcotDataRetrieverJob,
    persistance::db::{DatabaseClient, DatabaseConnectionParameters},
    schema::errors::Error,
};

const MONTHS: [&'static str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[tokio::main]
async fn main() -> Result<(), Error> {
    let data_directory = std::env::var("DATA_DIRECTORY")
        .unwrap_or_else(|_| "/Users/evancoulson/hydrogen-trading/data".to_string());

    let mut client = DatabaseClient::open(&DatabaseConnectionParameters::new(
        "hydrogen_trading_dev",
        "localhost",
        "hydrogen_trading",
    ))
    .await?;

    for month in MONTHS {
        let input = ErcotDataRetrieverJob::extract(&data_directory, month).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
        let generations = ErcotDataRetrieverJob::transform(input).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

        ErcotDataRetrieverJob::load(&mut client, generations)
            .await
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                process::exit(1);
            });
    }

    println!("Completed data retrieval");

    Ok(())
}
