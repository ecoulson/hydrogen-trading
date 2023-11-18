use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct QueryParameters<'r> {
    parameters: HashMap<&'r str, Vec<&'r str>>,
}

enum ParseState<'n> {
    Key,
    Value(&'n str),
}

impl<'r> QueryParameters<'r> {
    pub fn parse<'n>(query: &'n str) -> QueryParameters<'n> {
        let mut parameters = HashMap::new();
        let mut start = 0;
        let mut state = ParseState::Key;

        for (i, ch) in query.chars().enumerate() {
            match state {
                ParseState::Key => {
                    let key = &query[start..i];

                    if ch == '&' {
                        start = i + 1;

                        if !parameters.contains_key(key) {
                            parameters.insert(key, vec![]);
                        }
                    }

                    if ch == '=' {
                        start = i + 1;
                        state = ParseState::Value(key);

                        if !parameters.contains_key(key) {
                            parameters.insert(key, vec![]);
                        }
                    }
                }
                ParseState::Value(key) => {
                    let value = &query[start..i];

                    if ch == '&' {
                        start = i + 1;
                        state = ParseState::Key;

                        if let Some(values) = parameters.get_mut(key) {
                            values.push(value);
                        }
                    }
                }
            }
        }

        match state {
            ParseState::Key => (),
            ParseState::Value(key) => {
                if let Some(values) = parameters.get_mut(key) {
                    values.push(&query[start..query.len()]);
                }
            }
        }

        QueryParameters { parameters }
    }

    pub fn get(&self, key: &str) -> Option<&Vec<&'r str>> {
        self.parameters.get(key)
    }

    pub fn get_one(&self, key: &str) -> Option<&&'r str> {
        if let Some(x) = self.parameters.get(key) {
            return x.first()
        }
        
        None
    }
}
