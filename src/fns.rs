use std::{
    fs::{self, File},
    io::{BufReader, Write},
    path::Path,
};

use serde_json::Value;

use crate::{
    types::{Module, Studien},
    vars::{CACHE_PATH, MODULES_PATH, STUDIES_PATH},
};

#[allow(dead_code)]
pub fn encode_html(html: &str) -> String {
    html.replace("\"", "&quot;")
        .replace("&", "&amp;")
        .replace("'", "&#x27;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

pub fn decode_html(html: &str) -> String {
    html.replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&#x27;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

pub fn to_alphanum(s: &str) -> String {
    s.to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

pub fn encode_url(url: &str) -> String {
    // WARN: elaborate encoding not suited for the feeble mortal mind
    url.replace(" ", "%20")
}

pub fn create_write_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path);
    path.parent()
        .and_then(|parent| fs::create_dir_all(parent).ok());
    fs::File::create(path).and_then(|mut f| f.write_all(content.as_bytes()))
}

pub fn fetch_to_file(url: &str, path: &str) -> String {
    println!("fetching {} to {}", url, path);

    // TODO: log errors
    let r = minreq::get(encode_url(url)).send().unwrap();
    let html = r.as_str().unwrap();

    create_write_file(path, html).unwrap_or_else(|_| println!("Couldn't cache data"));

    html.to_string()
}

pub fn fetch_or_cache(url: &str, subpath: &str) -> String {
    let pathstr = CACHE_PATH.to_owned() + subpath;
    let path = Path::new(&pathstr);
    if path.exists()
        && let Ok(c) = fs::read_to_string(path)
    {
        println!("using cached {}", pathstr);
        c
    } else {
        fetch_to_file(url, &pathstr)
    }
}

// don't mind the duplicated logic

pub fn read_bloat_into_mem() -> (Vec<Studien>, Vec<Module>) {
    (pawse(STUDIES_PATH), pawse(MODULES_PATH))
}

pub fn read_bloat_into_mem_untyped() -> (Vec<Value>, Vec<Value>) {
    (pawse(STUDIES_PATH), pawse(MODULES_PATH))
}

fn pawse<T: serde::de::DeserializeOwned>(path: &str) -> Vec<T> {
    fs::read_dir(CACHE_PATH.to_owned() + path)
        .unwrap_or_else(|_| panic!("Cached path doesn't exist: {}", path))
        .filter_map(|s| {
            // TODO: log errors
            s.ok().and_then(|s| {
                File::open(s.path())
                    .ok()
                    .and_then(|f| serde_json::from_reader(BufReader::new(f)).ok())
            })
        })
        .collect()
}

pub fn uppercase_first_letter(s: &str) -> String {
    match s.chars().next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + &s[1..],
    }
}

pub fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_lower = false;

    for ch in s.chars() {
        if ch == '_' {
            out.push('_');
            prev_lower = false;
            continue;
        }
        if ch.is_ascii_uppercase() {
            if prev_lower && !out.ends_with('_') {
                out.push('_');
            }
            out.extend(ch.to_ascii_lowercase().to_string().chars());
            prev_lower = false;
        } else {
            out.push(ch);
            prev_lower = ch.is_ascii_lowercase() || ch.is_ascii_digit();
        }
    }
    out
}
