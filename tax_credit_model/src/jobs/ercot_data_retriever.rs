use crate::{
    parsers::csv_parser::{CsvParser, CsvRow},
    schema::{
        ercot::{ErcotFuelMix, ErcotRTMPrice, SettlementPointLocation},
        errors::Error,
        simulation_schema::{EnergySourcePortfolio, GenerationMetric},
        time::Timestamp,
    },
};
use chrono::NaiveDateTime;
use std::collections::HashMap;

const HOURS_PER_DAY: u8 = 24;
const SEGMENTS_PER_HOUR: u8 = 4;
const DATE_FORMAT: &'static str = "%m/%d/%Y %H:%M";

// 2023 link: https://www.ercot.com/files/docs/2023/02/07/IntGenbyFuel2023.xlsx
pub struct ErcotDataRetrieverJob {}

type Result<T> = std::result::Result<T, Error>;

pub struct ErcotDataRetrieverInput {
    fuel_mix_rows: Vec<CsvRow>,
    rtm_pricing_rows: Vec<CsvRow>,
}

impl ErcotDataRetrieverJob {
    pub fn extract(data_directory: &str) -> Result<ErcotDataRetrieverInput> {
        let fuel_mix_path = format!("{}{}", data_directory, "/ercot/fuel_mix_2023_01.csv");
        let rtm_pricing_path = format!("{}{}", data_directory, "/ercot/rtm_pricing_2023_01.csv");

        Ok(ErcotDataRetrieverInput {
            fuel_mix_rows: CsvParser::parse(&fuel_mix_path)?,
            rtm_pricing_rows: CsvParser::parse(&rtm_pricing_path)?,
        })
    }

    pub fn transform(input: ErcotDataRetrieverInput) -> Result<Vec<GenerationMetric>> {
        let fuel_mixes = ErcotDataRetrieverJob::transform_fuel_mixes(input.fuel_mix_rows)?;
        let rtm_prices = ErcotDataRetrieverJob::transform_rtm_prices(input.rtm_pricing_rows)?;

        ErcotDataRetrieverJob::form_generations(fuel_mixes, rtm_prices)
    }

    fn transform_fuel_mixes(
        fuel_mix_rows: Vec<CsvRow>,
    ) -> Result<HashMap<Timestamp, Vec<ErcotFuelMix>>> {
        let mut fuel_mixes: HashMap<Timestamp, Vec<ErcotFuelMix>> = HashMap::new();

        for row in fuel_mix_rows {
            for i in 0..HOURS_PER_DAY * SEGMENTS_PER_HOUR {
                let hour = i / SEGMENTS_PER_HOUR;
                let segment = i % SEGMENTS_PER_HOUR + 1;
                let segment_string =
                    &format!("{}:{}", hour, ErcotDataRetrieverJob::get_segment(segment)?);
                let formatted_date: String = row.get_column("Date")?;
                let date = NaiveDateTime::parse_from_str(
                    &ErcotDataRetrieverJob::build_date(&formatted_date, hour, segment)?,
                    DATE_FORMAT,
                )
                .map_err(|_| Error::create_parse_error(&formatted_date))?;
                let fuel_mix = ErcotFuelMix {
                    date: Timestamp::new(date.timestamp(), date.timestamp_subsec_nanos()),
                    fuel_source: row.get_column("Fuel")?,
                    settlement: row.get_column("Settlement Type")?,
                    electricity_produced: ErcotDataRetrieverJob::parse_float(
                        &row.get_column::<String>(segment_string)?,
                    )?,
                };

                if let Some(fuels) = fuel_mixes.get_mut(&fuel_mix.date) {
                    fuels.push(fuel_mix);
                } else {
                    fuel_mixes.insert(fuel_mix.date, vec![fuel_mix]);
                }
            }
        }

        Ok(fuel_mixes)
    }

    fn transform_rtm_prices(rtm_pricing_rows: Vec<CsvRow>) -> Result<Vec<ErcotRTMPrice>> {
        let mut rtm_prices: Vec<ErcotRTMPrice> = vec![];

        for row in rtm_pricing_rows {
            let formatted_date = &ErcotDataRetrieverJob::build_date(
                &row.get_column::<String>("Delivery Date")?,
                row.get_column::<u8>("Delivery Hour")? - 1,
                row.get_column("Delivery Interval")?,
            )?;
            let date = NaiveDateTime::parse_from_str(formatted_date, DATE_FORMAT)
                .map_err(|_| Error::create_parse_error(formatted_date))?;
            rtm_prices.push(ErcotRTMPrice {
                delivery_timestamp: Timestamp::new(date.timestamp(), date.timestamp_subsec_nanos()),
                repeated_hour_flag: row.get_column("Repeated Hour Flag")?,
                settlement_point_location: row.get_column("Settlement Point Name")?,
                settlement_point_type: row.get_column("Settlement Point Type")?,
                settlement_point_price: ErcotDataRetrieverJob::parse_float(
                    &row.get_column::<String>("Settlement Point Price")?,
                )?,
            })
        }

        Ok(rtm_prices)
    }

    fn form_generations(
        fuel_mixes: HashMap<Timestamp, Vec<ErcotFuelMix>>,
        rtm_prices: Vec<ErcotRTMPrice>,
    ) -> Result<Vec<GenerationMetric>> {
        rtm_prices
            .iter()
            .filter(|price| price.settlement_point_location == SettlementPointLocation::HubAverage)
            .map(|price| {
                let portfolio = fuel_mixes
                    .get(&price.delivery_timestamp)
                    .ok_or_else(|| Error::create_not_found_error(""))?
                    .iter()
                    .fold(
                        EnergySourcePortfolio::default(),
                        |mut portfolio, fuel_mix| {
                            portfolio
                                .add_energy(&fuel_mix.fuel_source, fuel_mix.electricity_produced);

                            portfolio
                        },
                    );

                Ok(GenerationMetric {
                    plant_id: 0,
                    time_generated: price.delivery_timestamp.clone(),
                    portfolio,
                    sale_price_usd_per_mwh: price.settlement_point_price,
                })
            })
            .collect()
    }

    fn build_date(date: &str, hour: u8, segment: u8) -> Result<String> {
        Ok(format!(
            "{} {}:{}",
            date,
            format!("{:02}", hour),
            ErcotDataRetrieverJob::get_segment(segment)?
        ))
    }

    fn get_segment(segment: u8) -> Result<&'static str> {
        match segment {
            1 => Ok("00"),
            2 => Ok("15"),
            3 => Ok("30"),
            4 => Ok("45"),
            _ => Err("Illegal segment"),
        }
        .map_err(|_| Error::create_parse_error("Should be valid segment"))
    }

    fn parse_float(value: &str) -> Result<f32> {
        let cleaned_float = value.replace(",", "");

        cleaned_float
            .parse()
            .map_err(|_err| Error::create_parse_error(value))
    }

    pub fn load(_generations: Vec<GenerationMetric>) -> Result<()> {
        // make request
        Ok(())
    }
}
