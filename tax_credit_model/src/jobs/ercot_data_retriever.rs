use crate::schema::{simulation_schema::GenerationMetric, time::Timestamp};

// 2023 link: https://www.ercot.com/files/docs/2023/02/07/IntGenbyFuel2023.xlsx
pub struct ErcotDataRetrieverJob {
    data_directory: String,
}

impl ErcotDataRetrieverJob {
    pub fn new(file_path: &str) -> ErcotDataRetrieverJob {
        ErcotDataRetrieverJob {
            data_directory: String::from(file_path),
        }
    }

    pub fn run(&mut self) {
        self.data_directory.push_str("/ercot/fuel_mix_2023_01.csv");
        let csv = std::fs::read_to_string(&self.data_directory).expect("CSV should exist");
        let mut lines = csv.lines();
        let mut generations: Vec<GenerationMetric> = vec![];
        lines.next(); // Ignore csv heading

        for line in lines {
            let mut values: Vec<String> = Vec::with_capacity(100);
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;

            while i < chars.len() {
                let (next_index, value) = if chars[i] == '"' {
                    self.parse_to_quotation(&chars, i)
                } else {
                    self.parse_to_seperator(&chars, i)
                };

                values.push(value);
                i = next_index + 1;
            }

            // should create an ErcotFuelMix struct
            // should create an ErcotMarketPrice struct
            // aggregate the retrieved data in to generation metrics
            // then we can have power plant specific aggregations and have the simulation work as
            // expected
            let generation = GenerationMetric {
                plant_id: 0,
                amount_mwh: 0.0, // can get from data
                amount_mmbtu: 0.0, // depends on type but can calc
                sale_price_usd_per_mwh: 0.0, // need market data
                time_generated: Timestamp::default()
            };
        }
    }

    fn parse_to_quotation(&self, chars: &Vec<char>, i: usize) -> (usize, String) {
        let mut i = i + 1;
        let mut value = String::new();

        while i < chars.len() && chars[i] != '"' {
            value.push(chars[i]);
            i += 1;
        }

        (i + 1, value)
    }

    fn parse_to_seperator(&self, chars: &Vec<char>, i: usize) -> (usize, String) {
        let mut i = i;
        let mut value = String::new();

        while i < chars.len() && chars[i] != ',' {
            value.push(chars[i]);
            i += 1;
        }

        (i, value)
    }
}
