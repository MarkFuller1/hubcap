use std::clone::Clone;
use std::fmt;
use std::fmt::Formatter;
use std::iter::Iterator;

static HTTP_DESCRIPTIONS: std::collections::HashMap<String, String> =
    [("200", "OK"),
        ("201", "Created"),
        ("202", "Accepted"),
        ("203", "Non-Authoritative Information"),
        ("204", "No Content"),
        ("205", "Reset Content"),
        ("206", "Partial Content"),
    ].iter().cloned().collect();


pub struct HttpResponse {
    version: String,
    code: u8,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        _ = write!(f, "HTTP/");
        _ = write!(f, " {}", self.version);
        _ = match HTTP_DESCRIPTIONS.get(&self.version) {
            Some(code) => write!(f, " {code}"),
            None => println!("{code} ; is not a configured http code")
        };
        _ = writeln!("");


    }

    fn write_headers(map: &mut std::collections::HashMap<String, String>, f: &mut Formatter<'_>) {
        map.retain(|key, value| {
            println!("{} / {}", key, value);

            !key.starts_with("a")
        })
    }
}
