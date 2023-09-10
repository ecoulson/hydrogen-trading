use std::{collections::HashMap, num::ParseFloatError};

use chrono::NaiveDate;

use crate::{parsers::csv_parser::CsvParser, schema::simulation_schema::EnergySource};

// 2023 link: https://www.ercot.com/files/docs/2023/02/07/IntGenbyFuel2023.xlsx
pub struct ErcotDataRetrieverJob {
    data_directory: String,
}

#[derive(Default, Debug, PartialEq)]
enum Settlement {
    #[default]
    Final,
}

#[derive(Default, Debug, PartialEq)]
struct ErcotFuelMix {
    date: NaiveDate,
    total_electricity: f32,
    fuel_source: EnergySource,
    settlement: Settlement,
    electricity_production: Vec<f32>,
}

impl ErcotDataRetrieverJob {
    pub fn new(file_path: &str) -> ErcotDataRetrieverJob {
        ErcotDataRetrieverJob {
            data_directory: String::from(file_path),
        }
    }

    pub fn run(&self) {
        let mut fuel_mixes: Vec<ErcotFuelMix> = vec![];
        let fuel_mix_path = format!("{}{}", &self.data_directory, "/ercot/fuel_mix_2023_01.csv");
        let rtm_pricing_path = format!("{}{}", &self.data_directory, "/ercot/rtm_pricing_2023_01.csv");
        let fuel_mix_entries = CsvParser::parse(&fuel_mix_path);
        let rtm_pricing_entries = CsvParser::parse(&rtm_pricing_path);

        for entry in fuel_mix_entries {
            let fuel_source = match entry["Fuel"].as_str() {
                "Coal" => Ok(EnergySource::Coal),
                "Biomass" => Ok(EnergySource::Biomass),
                "Gas" => Ok(EnergySource::NaturalGas),
                "Gas-CC" => Ok(EnergySource::NaturalGas),
                "Hydro" => Ok(EnergySource::Hydropower),
                "Nuclear" => Ok(EnergySource::Nuclear),
                "Solar" => Ok(EnergySource::Solar),
                "Wind" => Ok(EnergySource::Wind),
                "WSL" => Ok(EnergySource::WholesaleStoageLoad),
                "Other" => Ok(EnergySource::Unknown),
                _ => Err("Invalid fuel source"),
            }
            .unwrap();

            let settlement = match entry["Settlement Type"].as_str() {
                "FINAL" => Ok(Settlement::Final),
                _ => Err("Invalid settlement type"),
            }
            .unwrap();

            fuel_mixes.push(ErcotFuelMix {
                date: NaiveDate::parse_from_str(&entry["Date"], "%m/%d/%Y")
                    .expect("Should be valid date"),
                fuel_source,
                settlement,
                total_electricity: self
                    .parse_float(&entry["Total"])
                    .expect("Should be a valid float"),
                electricity_production: self.parse_electricity_production(&entry),
            });
        }

        dbg!(rtm_pricing_entries);
    }

    fn parse_electricity_production(&self, entry: &HashMap<String, String>) -> Vec<f32> {
        let mut productions = vec![];
        let mut i = 1;

        while i < 24 * 4 {
            let hour = i / 4;
            let segment = i % 4;
            let minute = match segment {
                0 => Ok("00"),
                1 => Ok("15"),
                2 => Ok("30"),
                3 => Ok("45"),
                _ => Err("Illegal segment"),
            }
            .unwrap();
            let key = format!("{}:{}", hour, minute);

            productions.push(self.parse_float(&entry[&key]).expect("Should be a float"));

            i += 1;
        }

        productions.push(self.parse_float(&entry["0:00"]).expect("Should be a float"));

        productions
    }

    fn parse_float(&self, value: &str) -> Result<f32, ParseFloatError> {
        let cleaned_float = value.replace(",", "");

        cleaned_float.parse()
    }
}
