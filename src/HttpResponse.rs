use std::clone::Clone;
use std::fmt;
use std::fmt::Formatter;
use std::iter::Iterator;

use once_cell::sync::Lazy;

static HTTP_DESCRIPTIONS: Lazy<std::collections::HashMap<String, String>> = Lazy::new(|| {
    [
        ("200".to_string(), "OK".to_string()),
        ("201".to_string(), "Created".to_string()),
        ("202".to_string(), "Accepted".to_string()),
        ("203".to_string(), "Non-Authoritative Information".to_string()),
        ("204".to_string(), "No Content".to_string()),
        ("205".to_string(), "Reset Content".to_string()),
        ("206".to_string(), "Partial Content".to_string()),
        ("404".to_string(), "Not Found".to_string()),
    ]
        .iter()
        .cloned()
        .collect()
});


pub struct HttpResponse {
    pub(crate) version: String,
    pub(crate) code: String,
    pub(crate) headers: std::collections::HashMap<String, String>,
    pub(crate) body: String,
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let r = write!(f, "{} {} {}\n", self.version, self.code ,HTTP_DESCRIPTIONS.get(&self.code).unwrap_or(&"Unknown".to_string()));
        match r {
            Err(e) => println!("{}", e),
            _ => {print!("")}
        }

        for (key, value) in &self.headers {
            write!(f, "{}: {}\n", key, value)?;
        }

        write!(f, "{}", self.body)
    }
}