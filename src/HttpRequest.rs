use std::str::FromStr;

pub struct HttpRequest {
    pub(crate) method: String,
    pub(crate) path: String,
    pub(crate) version: String,
    pub(crate) headers: std::collections::HashMap<String, String>,
    pub(crate) body: String,
}

impl FromStr for HttpRequest {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let lines: Vec<&str> = s.split("\r\n").collect();

        if lines.is_empty() {
            return Err("there were not lines to parse");
        }

        // parse method
        let first_line = lines.get(0).unwrap();
        let first_line_tokens: Vec<&str> = first_line.split(' ').collect();

        if first_line_tokens.is_empty() || first_line_tokens.len() != 3 {
            return Err("There were not three tokens in the first line");
        }

        let method: String = first_line_tokens.get(0).unwrap().to_string();

        // parse path
        let path: String = first_line_tokens.get(1).unwrap().to_string();

        // parse version
        let version = first_line_tokens.get(2).unwrap().to_string();

        // parse headers
        let mut headers: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let mut lines_index = 1;

        while lines_index < lines.len() && !lines[lines_index].is_empty() {
            let keyValue: Vec<&str> = lines[lines_index].split(':').collect();
            if keyValue.len() == 2 {
                headers.insert(keyValue[0].trim().to_string(), keyValue[1].trim().to_string());
            }
            lines_index += 1;
        }

        // parse body
        let mut body = String::new();
        for i in lines_index..lines.len() {
            body.push_str(lines[i]);
            body.push_str("\n");
        }

        Ok(HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}