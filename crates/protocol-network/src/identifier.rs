use std::fmt::Display;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NAMESPACE_REGEX: Regex = Regex::new(r"^[a-z0-9.-_]+").unwrap();
    static ref PATH_REGEX: Regex = Regex::new(r"^[a-z0-9.-_/]+").unwrap();
}

pub struct Identifier<'a> {
    pub namespace: &'a str,
    pub path: &'a str,
}

impl<'a> Display for Identifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl<'a> From<&'a str> for Identifier<'a> {
    fn from(value: &'a str) -> Self {
        if value.contains(":") {
            let parts: Vec<&str> = value.split(":").collect();
            Self {
                namespace: parts[0],
                path: parts[1]
            }
        } else {
            Self {
                namespace: "minecraft",
                path: value
            }
        }
    }
}

impl<'a> Identifier<'a> {
    pub fn new(namespace: &'a str, path: &'a str) -> Option<Self> {
        if !NAMESPACE_REGEX.is_match(namespace) || !PATH_REGEX.is_match(path) {
            None
        } else {
            Some(Self { namespace, path })
        }
    }
}