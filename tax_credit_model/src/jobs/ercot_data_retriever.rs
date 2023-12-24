use crate::{
    parsers::excel::{ExcelSheet, ExcelWorkbook},
    persistance::grid::GridClient,
    schema::{
        ercot::{ErcotFuelMix, ErcotRTMPrice, SettlementPointLocation},
        errors::{Error, Result},
        simulation::{EnergySourcePortfolio, GenerationMetric},
        time::Timestamp,
    },
};
use chrono::{Duration, NaiveDateTime};
use std::collections::HashMap;

const DATE_COLUNMN: usize = 0;
const FUEL_SOURCE_COLUMN: usize = 1;
const SETTLEMENT_COLUMN: usize = 2;
const DAY_START_COLUNMN: usize = 4;
const DAY_END_COLUNMN: usize = 99;
const DELIVERY_HOUR_COLUMN: usize = 1;
const DELIVERY_INTERVAL_COLUMN: usize = 2;
const REPEATED_HOUR_FLAG_COLUMN: usize = 3;
const SETTLEMENT_POINT_NAME_COLUMN: usize = 4;
const SETTLEMENT_POINT_TYPE_COLUMN: usize = 5;
const SETTLEMENT_POINT_PRICE_COLUMN: usize = 6;
const MINUTES_PER_INTERVAL: f64 = 15.0;

// 2023 link: https://www.ercot.com/files/docs/2023/02/07/IntGenbyFuel2023.xlsx
pub struct ErcotDataRetrieverJob {}

pub struct ErcotDataRetrieverInput {
    fuel_mix_sheet: ExcelSheet,
    rtm_pricing_sheet: ExcelSheet,
}

impl ErcotDataRetrieverJob {
    pub fn extract(data_directory: &str, month: &str) -> Result<ErcotDataRetrieverInput> {
        let fuel_mix_path = format!("{}{}", data_directory, "/ercot/fuel_mix_2023.xlsx");
        let rtm_price_path = format!("{}{}", data_directory, "/ercot/rtm_prices_2023.xlsx");
        let mut fuel_source_workbook = ExcelWorkbook::open(&fuel_mix_path)?;
        let mut rtm_price_workbook = ExcelWorkbook::open(&rtm_price_path)?;

        let rtm_prices_sheet = rtm_price_workbook.get_sheet(&month)?;
        let fuel_mix_sheet = fuel_source_workbook.get_sheet(&month)?;

        Ok(ErcotDataRetrieverInput {
            fuel_mix_sheet,
            rtm_pricing_sheet: rtm_prices_sheet,
        })
    }

    pub fn transform(input: ErcotDataRetrieverInput) -> Result<Vec<GenerationMetric>> {
        let fuel_mixes = ErcotDataRetrieverJob::transform_fuel_mixes(input.fuel_mix_sheet)?;
        let rtm_prices = ErcotDataRetrieverJob::transform_rtm_prices(input.rtm_pricing_sheet)?;

        ErcotDataRetrieverJob::form_generations(fuel_mixes, rtm_prices)
    }

    fn transform_fuel_mixes(
        fuel_mix_sheet: ExcelSheet,
    ) -> Result<HashMap<Timestamp, Vec<ErcotFuelMix>>> {
        let mut fuel_mixes: HashMap<Timestamp, Vec<ErcotFuelMix>> = HashMap::new();
        let mut fuel_mix_rows = fuel_mix_sheet.rows();
        fuel_mix_rows.next();

        for row in fuel_mix_rows {
            let mut date = row.get_date(DATE_COLUNMN)?;
            let fuel_source = row.get_string(FUEL_SOURCE_COLUMN)?;
            let settlement = row.get_string(SETTLEMENT_COLUMN)?;

            for interval_column in DAY_START_COLUNMN..=DAY_END_COLUNMN {
                if row.is_empty_cell(interval_column)? {
                    continue;
                }
                date += Duration::minutes(15);
                let fuel_mix = ErcotFuelMix {
                    electricity_produced: row.get_float(interval_column)?,
                    settlement: settlement.parse()?,
                    fuel_source: fuel_source.parse()?,
                    date: Timestamp::from(date),
                };

                if let Some(fuel_mix_list) = fuel_mixes.get_mut(&fuel_mix.date) {
                    fuel_mix_list.push(fuel_mix);
                } else {
                    fuel_mixes.insert(fuel_mix.date, vec![fuel_mix]);
                }
            }
        }

        Ok(fuel_mixes)
    }

    fn transform_rtm_prices(rtm_prices_sheet: ExcelSheet) -> Result<Vec<ErcotRTMPrice>> {
        let mut rtm_prices: Vec<ErcotRTMPrice> = vec![];
        let mut rtm_prices_rows = rtm_prices_sheet.rows();
        rtm_prices_rows.next();

        for row in rtm_prices_rows {
            let mut date = NaiveDateTime::parse_from_str(
                &format!(
                    "{} {}:{}",
                    row.get_string(DATE_COLUNMN)?,
                    format!("{:02}", row.get_float(DELIVERY_HOUR_COLUMN)? - 1.0),
                    format!(
                        "{:02}",
                        (row.get_float(DELIVERY_INTERVAL_COLUMN)? - 1.0) * MINUTES_PER_INTERVAL
                    ),
                ),
                "%m/%d/%Y %0H:%0M",
            )
            .map_err(|err| Error::invalid_argument(&err.to_string()))?;
            date += Duration::minutes(15);
            rtm_prices.push(ErcotRTMPrice {
                delivery_timestamp: Timestamp::from(date),
                repeated_hour_flag: row.get_string(REPEATED_HOUR_FLAG_COLUMN)?.parse()?,
                settlement_point_type: row.get_string(SETTLEMENT_POINT_TYPE_COLUMN)?.parse()?,
                settlement_point_location: row.get_string(SETTLEMENT_POINT_NAME_COLUMN)?.parse()?,
                settlement_point_price: row.get_float(SETTLEMENT_POINT_PRICE_COLUMN)?,
            });
        }

        Ok(rtm_prices)
    }

    fn form_generations(
        fuel_mixes: HashMap<Timestamp, Vec<ErcotFuelMix>>,
        rtm_prices: Vec<ErcotRTMPrice>,
    ) -> Result<Vec<GenerationMetric>> {
        rtm_prices
            .iter()
            .filter(|price| {
                price.settlement_point_location == SettlementPointLocation::HubAverage
                    && fuel_mixes.contains_key(&price.delivery_timestamp)
            })
            .map(|price| {
                let portfolio = fuel_mixes
                    .get(&price.delivery_timestamp)
                    .ok_or_else(|| Error::not_found("No timestamp"))?
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

    pub fn load(
        generations: Vec<GenerationMetric>,
        grid_client: &Box<dyn GridClient>,
    ) -> Result<()> {
        grid_client.add_generations(generations)
    }
}
