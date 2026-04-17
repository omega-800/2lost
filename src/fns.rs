use std::{fs, io::Write, path::Path};

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

pub fn fetch_to_file(url: &str, path: &Path) -> String {
    println!(
        "fetching {} to {}",
        url,
        path.as_os_str().to_str().unwrap_or("")
    );

    // FIXME: encode filename special chars
    let r = minreq::get(url).send().unwrap();
    let html = r.as_str().unwrap();

    path.parent()
        .and_then(|parent| fs::create_dir_all(parent).ok());
    fs::File::create(path)
        .and_then(|mut f| f.write_all(html.as_bytes()))
        .unwrap_or_else(|_| println!("Couldn't cache data"));
    html.to_string()
}

pub fn fetch_or_cache(url: &str, subpath: &str) -> String {
    let pathstr = "./.cache/".to_owned() + subpath;
    let path = Path::new(&pathstr);
    if path.exists()
        && let Ok(c) = fs::read_to_string(path)
    {
        println!("using cached {}", pathstr);
        c
    } else {
        fetch_to_file(url, path)
    }
}
