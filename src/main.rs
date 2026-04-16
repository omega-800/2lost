use std::{collections::HashMap, fs, io::Write, path::Path, str::from_utf8};

use serde_json::Value;

const BASE_URL: &str = "http://studien.ost.ch";

fn main() {
    let html = fetch_or_cache(BASE_URL, "html.html");
    let module_names = parse_modules(&html);
    let modules = fetch_modules(&module_names);
    let study_names = parse_studies(&modules);
    let studies = fetch_studies(&study_names);
}

// TODO: parallelize

fn fetch_to_file(url: &str, path: &Path) -> String {
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

fn fetch_or_cache(url: &str, subpath: &str) -> String {
    let pathstr = "./.cache/".to_owned() + subpath;
    let path = Path::new(&pathstr);
    if path.exists()
        && let Ok(c) = fs::read_to_string(path)
    {
        c
    } else {
        fetch_to_file(url, path)
    }
}

fn fetch_studies<'a>(study_names: &Vec<&'a str>) -> HashMap<&'a str, Value> {
    study_names
        .iter()
        .filter_map(|m| {
            serde_json::from_str(&fetch_or_cache(
                &(BASE_URL.to_owned() + "/allStudies/" + m),
                &("studies/".to_owned() + m),
            ))
            .ok()
            .map(|v| (*m, v))
        })
        .collect()
}

// with .json
fn parse_studies<'a>(modules: &'a HashMap<&str, Value>) -> Vec<&'a str> {
    modules
        .iter()
        .filter_map(|(_, v)| {
            if let Value::Object(v) = v
                && v.contains_key("zuordnungen")
                && let Some(Value::Array(v)) = v.get("zuordnungen")
            {
                return Some(
                    v.iter()
                        .filter_map(|i| {
                            if let Value::Object(i) = i
                                && i.contains_key("url")
                                && let Some(Value::String(i)) = i.get("url")
                            {
                                return Some(&i.as_str()[11..]);
                            }
                            None
                        })
                        .collect::<Vec<&str>>(),
                );
            }
            None
        })
        .flatten()
        .collect()
}

fn fetch_modules<'a>(module_names: &Vec<&'a str>) -> HashMap<&'a str, Value> {
    module_names
        .iter()
        .filter_map(|m| {
            serde_json::from_str(&fetch_or_cache(
                &(BASE_URL.to_owned() + "/allModules/" + m + ".json"),
                &("modules/".to_owned() + m + ".json"),
            ))
            .ok()
            .map(|v| (*m, v))
        })
        .collect()
}

// without .json
fn parse_modules(html: &str) -> Vec<&str> {
    html.match_indices("href=\"allModules/")
        .filter_map(|(i, _)| {
            let f = i + 17;
            let h = html.as_bytes();
            if f < h.len()
                && let Some(t) = &h[f..].iter().position(|&b| b == b'"')
                && let Ok(res) = from_utf8(&h[f..(t + f - 5)])
            {
                return Some(res);
            }
            None
        })
        .collect()
}

// ls *.json | xargs yq -oy '. | keys' | sort | uniq -c | sort -bgr
// 4451 - zustand
// 4451 - sprache
// 4451 - semesterBewertung
// 4451 - kuerzel
// 4451 - id
// 4451 - bezeichnung
// 4450 - kreditpunkte
// 4438 - orte
// 4425 - kurse
// 4406 - zuordnungen
// 4212 - uebersetzungen
// 4148 - dozenten
// 4031 - lernziele
// 3830 - durchfuehrungen
// 2682 - skriptAblageLink
// 2237 - pruefung
// 1680 - vorgaenger
// 1680 - nachfolger
// 1605 - vorausgKenntnisse
// 1187 - empfehlungen
//  637 - voraussetzungen
//  562 - anschlussmodule
//  186 - mengen
//    8 - inhalt
//    3 - methoden
