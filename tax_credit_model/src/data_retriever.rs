use std::process::{self, exit};

use crate::jobs::ercot_data_retriever::ErcotDataRetrieverJob;

use crate::server::{Dependencies, ServerConfiguration};

const MONTHS: [&'static str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub fn fill_generations(configuration: ServerConfiguration, dependencies: &Dependencies) {
    let generations = dependencies
        .generation_client
        .list_generations()
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            exit(1)
        });

    if !generations.is_empty() {
        ErcotDataRetrieverJob::load(generations, &dependencies.grid_client).unwrap_or_else(|err| {
            eprintln!("{err}");
            exit(1)
        });
        return;
    }

    for month in MONTHS {
        let input = ErcotDataRetrieverJob::extract(&configuration.data_directory, month)
            .unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(1);
            });
        let generations = ErcotDataRetrieverJob::transform(input).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

        if generations.is_empty() {
            continue;
        }

        for generation in &generations {
            dependencies
                .generation_client
                .create_generation(generation)
                .unwrap_or_else(|err| {
                    eprintln!("{err}");
                    exit(1)
                });
        }

        ErcotDataRetrieverJob::load(generations, &dependencies.grid_client).unwrap_or_else(|err| {
            eprintln!("{err}");
            exit(1)
        });
    }
}
