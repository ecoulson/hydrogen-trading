use std::{collections::HashMap, str::FromStr};

use crate::schema::errors::{Error, Result};

pub struct CsvRow {
    row: HashMap<String, String>,
}

impl CsvRow {
    pub fn get_column<T>(&self, key: &str) -> Result<T>
    where
        T: FromStr,
    {
        if let Some(value) = self.row.get(key) {
            value.parse().map_err(|_| Error::create_parse_error(value))
        } else {
            Err(Error::create_not_found_error(key))
        }
    }
}

pub struct CsvParser;

impl CsvParser {
    pub fn parse(path: &str) -> Result<Vec<CsvRow>> {
        let content =
            std::fs::read_to_string(&path).map_err(|_| Error::create_not_found_error(path))?;
        let mut lines = content.lines();
        let mut headers: Vec<String> = vec![];
        let mut rows = vec![];

        let heading = lines
            .next()
            .ok_or_else(|| Error::create_not_found_error("Could not find a heading line"))?;
        let chars: Vec<char> = heading.chars().collect();
        let mut header_index = 0;

        while header_index < chars.len() {
            let (next_index, value) = CsvParser::parse_value(&chars, header_index);

            headers.push(value);
            header_index = next_index + 1;
        }

        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            let mut cursor = 0;
            let mut value_index = 0;
            let mut row: HashMap<String, String> = HashMap::new();

            while cursor < chars.len() {
                let (next_index, value) = CsvParser::parse_value(&chars, cursor);
                row.insert(headers[value_index].to_string(), value);
                cursor = next_index + 1;
                value_index += 1;
            }

            rows.push(CsvRow { row });
        }

        Ok(rows)
    }

    fn parse_value(chars: &Vec<char>, i: usize) -> (usize, String) {
        let (i, mut value) = if chars[i] == '"' {
            CsvParser::parse_to_quotation(&chars, i)
        } else {
            CsvParser::parse_to_seperator(&chars, i)
        };

        value = value.chars().filter(|c| c.is_ascii()).collect::<String>();

        (i, value)
    }

    fn parse_to_quotation(chars: &Vec<char>, i: usize) -> (usize, String) {
        let mut i = i + 1;
        let mut value = String::new();

        while i < chars.len() && chars[i] != '"' {
            value.push(chars[i]);
            i += 1;
        }

        (i + 1, value)
    }

    fn parse_to_seperator(chars: &Vec<char>, i: usize) -> (usize, String) {
        let mut i = i;
        let mut value = String::new();

        while i < chars.len() && chars[i] != ',' {
            value.push(chars[i]);
            i += 1;
        }

        (i, value)
    }
}
