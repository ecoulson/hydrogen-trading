use tax_credit_model_server::jobs::ercot_data_retriever::ErcotDataRetrieverJob;

pub fn main() {
    let data_directory = std::env::var("DATA_DIRECTORY")
        .unwrap_or_else(|_| "/Users/evancoulson/hydrogen-trading/data".to_string());
    ErcotDataRetrieverJob::new(&data_directory).run();
}
