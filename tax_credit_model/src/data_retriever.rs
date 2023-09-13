use std::process;

use tax_credit_model_server::jobs::ercot_data_retriever::ErcotDataRetrieverJob;

pub fn main() {
    let data_directory = std::env::var("DATA_DIRECTORY")
        .unwrap_or_else(|_| "/Users/evancoulson/hydrogen-trading/data".to_string());

    let input = ErcotDataRetrieverJob::extract(&data_directory).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let generations = ErcotDataRetrieverJob::transform(input).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    ErcotDataRetrieverJob::load(generations).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("Completed data retrieval")
}
