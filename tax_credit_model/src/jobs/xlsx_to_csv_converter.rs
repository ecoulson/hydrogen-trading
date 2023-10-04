use std::{fs::File, io::BufReader};

use calamine::{open_workbook, DataType, Range, Reader, Rows, Xlsx, XlsxError};
use chrono::{Days, Duration, NaiveDate, NaiveDateTime, NaiveTime};

use crate::schema::{
    ercot::{ErcotFuelMix, ErcotRTMPrice},
    errors::{Error, Result},
    time::Timestamp,
};

const MONTHS: [&'static str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub fn convert_to_csv(data_directory: &str) -> Result<()> {
    let fuel_mix_path = format!("{}{}", data_directory, "/ercot/fuel_mix_2023.xlsx");
    let rtm_price_path = format!("{}{}", data_directory, "/ercot/rtm_prices_2023.xlsx");
    let mut fuel_source_workbook = ExcelWorkbook::open(&fuel_mix_path)?;
    let mut rtm_price_workbook = ExcelWorkbook::open(&rtm_price_path)?;
    let mut fuel_mixes: Vec<ErcotFuelMix> = vec![];
    let mut prices: Vec<ErcotRTMPrice> = vec![];

    for month in &MONTHS {
        let rtm_prices_sheet = rtm_price_workbook.get_sheet(&month)?;
        let fuel_mix_sheet = fuel_source_workbook.get_sheet(&month)?;
        let mut rtm_prices_rows = rtm_prices_sheet.rows();
        let mut fuel_mix_rows = fuel_mix_sheet.rows();
        rtm_prices_rows
            .next()
            .ok_or_else(|| Error::create_not_found_error("No heading was found in this sheet"))?;
        fuel_mix_rows
            .next()
            .ok_or_else(|| Error::create_not_found_error("No heading was found in this sheet"))?;

        for row in rtm_prices_rows {
            let date = NaiveDateTime::parse_from_str(
                &format!(
                    "{} {}:{}",
                    row.get_string(DATE_COLUNMN)?,
                    (row.get_float(DELIVERY_HOUR_COLUMN)? - 1.0),
                    (row.get_float(DELIVERY_INTERVAL_COLUMN)? - 1.0) * 15.0
                ),
                "%m/%d/%Y %H:%M",
            )
            .map_err(|err| Error::create_parse_error(&err.to_string()))?;

            prices.push(ErcotRTMPrice {
                delivery_timestamp: Timestamp::from(date),
                repeated_hour_flag: row.get_string(REPEATED_HOUR_FLAG_COLUMN)?.parse()?,
                settlement_point_type: row.get_string(SETTLEMENT_POINT_TYPE_COLUMN)?.parse()?,
                settlement_point_location: row.get_string(SETTLEMENT_POINT_NAME_COLUMN)?.parse()?,
                settlement_point_price: row.get_float(SETTLEMENT_POINT_PRICE_COLUMN)?,
            });
        }

        for row in fuel_mix_rows {
            let mut date = row.get_date(DATE_COLUNMN)?;
            let fuel_source = row.get_string(FUEL_SOURCE_COLUMN)?;
            let settlement = row.get_string(SETTLEMENT_COLUMN)?;

            for interval_column in DAY_START_COLUNMN..DAY_END_COLUNMN {
                if row.is_empty_cell(interval_column)? {
                    println!("Missing value");
                    continue;
                }

                date += Duration::minutes(15);
                fuel_mixes.push(ErcotFuelMix {
                    electricity_produced: row.get_float(interval_column)?,
                    settlement: settlement.parse()?,
                    fuel_source: fuel_source.parse()?,
                    date: Timestamp::from(date),
                });
            }
        }
    }

    Ok(())
}

