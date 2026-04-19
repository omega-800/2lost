use std::{fs, io::Write, path::Path};

use crate::vars::CACHE_PATH;

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

pub fn create_write_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path);
    path.parent()
        .and_then(|parent| fs::create_dir_all(parent).ok());
    fs::File::create(path).and_then(|mut f| f.write_all(content.as_bytes()))
}

pub fn fetch_to_file(url: &str, path: &str) -> String {
    println!("fetching {} to {}", url, path);

    // FIXME: encode filename special chars
    let r = minreq::get(url).send().unwrap();
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
