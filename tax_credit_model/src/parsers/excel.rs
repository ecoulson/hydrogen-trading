use std::{fs::File, io::BufReader};

use calamine::{open_workbook, DataType, Range, Reader, Rows, Xlsx, XlsxError};
use chrono::{Days, NaiveDate, NaiveDateTime, NaiveTime};

use crate::schema::errors::{Error, Result};

pub struct ExcelSheet {
    sheet: Range<DataType>,
}

impl ExcelSheet {
    pub fn new(sheet: Range<DataType>) -> ExcelSheet {
        ExcelSheet { sheet }
    }

    pub fn rows(&self) -> ExcelRows {
        ExcelRows {
            rows: self.sheet.rows(),
        }
    }
}

pub struct ExcelRows<'a> {
    rows: Rows<'a, DataType>,
}

impl<'a> Iterator for ExcelRows<'a> {
    type Item = ExcelRow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| ExcelRow::new(row))
    }
}

pub struct ExcelWorkbook {
    workbook: Xlsx<BufReader<File>>,
}

impl ExcelWorkbook {
    pub fn open(path: &str) -> Result<ExcelWorkbook> {
        Ok(ExcelWorkbook {
            workbook: open_workbook(path)
                .map_err(|err: XlsxError| Error::not_found(&err.to_string()))?,
        })
    }

    pub fn get_sheet(&mut self, sheet_name: &str) -> Result<ExcelSheet> {
        let sheet = self
            .workbook
            .worksheet_range(sheet_name)
            .ok_or_else(|| Error::not_found("Sheet not found in work book"))?
            .map_err(|err| Error::invalid_argument(&err.to_string()))?;

        Ok(ExcelSheet { sheet })
    }
}

pub struct ExcelRow<'a> {
    row: &'a [calamine::DataType],
}

impl<'a> ExcelRow<'a> {
    pub fn new<'b>(row: &'b [DataType]) -> ExcelRow<'b> {
        ExcelRow { row }
    }

    pub fn is_empty_cell(&self, column: usize) -> Result<bool> {
        Ok(self
            .row
            .get(column)
            .ok_or_else(|| Error::not_found("Column not found"))?
            .is_empty())
    }

    pub fn get_date(&self, column: usize) -> Result<NaiveDateTime> {
        let excel_epoch: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(1899, 12, 30).unwrap_or_else(|| panic!("Invalid date")),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap_or_else(|| panic!("Invalid time")),
        );
        let date = self
            .row
            .get(column)
            .ok_or_else(|| Error::not_found("Date column not found"))?;

        if let calamine::DataType::DateTime(serial_number) = date {
            Ok(excel_epoch
                .checked_add_days(Days::new(serial_number.clone() as u64))
                .ok_or_else(|| Error::invalid_argument("Invalid days to add"))?)
        } else {
            Err(Error::invalid_argument("Not a date time"))
        }
    }

    pub fn get_string(&self, column: usize) -> Result<&str> {
        self.row
            .get(column)
            .ok_or_else(|| Error::not_found("Settlement column not found"))?
            .get_string()
            .ok_or_else(|| Error::invalid_argument("Column is not a string"))
    }

    pub fn get_float(&self, column: usize) -> Result<f64> {
        self.row
            .get(column)
            .ok_or_else(|| Error::not_found("Settlement column not found"))?
            .get_float()
            .ok_or_else(|| Error::invalid_argument("Column is not a float"))
    }

    pub fn get_int(&self, column: usize) -> Result<i64> {
        self.row
            .get(column)
            .ok_or_else(|| Error::not_found("Settlement column not found"))?
            .get_int()
            .ok_or_else(|| Error::invalid_argument("Column is not a int"))
    }
}
