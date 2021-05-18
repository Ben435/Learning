use crate::hash_multi_map::HashMultiMap;
use std::fmt::{Display,Formatter,Result};

pub struct HttpRequest<'a> {
    pub request_method: HttpRequestMethod,
    pub request_path: &'a str,
    pub headers: HashMultiMap<String, &'a str>,
    pub body: &'a str,
}

#[derive(Debug)]
pub enum HttpRequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl Display for HttpRequestMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let str_repr = match self {
            HttpRequestMethod::GET => "GET",
            HttpRequestMethod::POST => "POST",
            HttpRequestMethod::PUT => "PUT",
            HttpRequestMethod::DELETE => "DELETE"
        };
        write!(f, "{}", str_repr)
    }
}

impl HttpRequestMethod {
    pub fn from_str(method: &str) -> Option<HttpRequestMethod> {
        match method {
            "GET" => Some(HttpRequestMethod::GET),
            "POST" => Some(HttpRequestMethod::POST),
            "PUT" => Some(HttpRequestMethod::PUT),
            "DELETE" => Some(HttpRequestMethod::DELETE),
            _ => None
        }
    }
}
