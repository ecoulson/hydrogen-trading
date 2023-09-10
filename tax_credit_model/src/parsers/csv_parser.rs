use std::collections::HashMap;

pub struct CsvParser;

impl CsvParser {
    pub fn parse(path: &str) -> Vec<HashMap<String, String>> {
        let content = std::fs::read_to_string(&path).expect("CSV should exist");
        let mut lines = content.lines();
        let mut csv: Vec<HashMap<String, String>> = vec![];
        let mut headers: Vec<String> = vec![];

        let heading = lines.next().expect("Should pull heading"); // Ignore csv heading
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
            let mut entry: HashMap<String, String> = HashMap::new();

            while cursor < chars.len() {
                let (next_index, value) = CsvParser::parse_value(&chars, cursor);
                entry.insert(headers[value_index].to_string(), value);
                cursor = next_index + 1;
                value_index += 1;
            }

            csv.push(entry);
        }

        csv
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
