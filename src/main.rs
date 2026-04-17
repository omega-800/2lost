use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufReader, Write},
    path::Path,
    str::from_utf8,
};

use serde_json::Value;

mod infer_types;
mod fns;
mod types;

use crate::{
    infer_types::{common_shape, shape_to_code, value_to_shape},
    fns::{decode_html, fetch_or_cache},
    types::Studies,
};

const BASE_URL: &str = "http://studien.ost.ch";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && &args[1] == "-s" {
        scrape();
    } else if args.len() > 1 && &args[1] == "-t" {
        let path = Path::new("./.cache/studies/10306_MiT.json");
        let f = File::open(path).unwrap();
        let r = BufReader::new(f);
        let t: Studies = serde_json::from_reader(r).unwrap();
    } else {
        println!("Usage: {} -[s|t]", &args[0]);
    }
}

fn scrape() {
    println!("Fetching html");
    let html = fetch_or_cache(BASE_URL, "html.html");

    println!("Parsing modules");
    let module_names = parse_modules(&html);
    println!("Fetching modules");
    let modules = fetch_modules(&module_names.iter().map(|s| s.as_str()).collect());
    println!("Typegen modules");
    gen_types(&modules, "modules");

    println!("Parsing studies");
    let study_names = parse_studies(&modules);
    println!("Fetching studies");
    let studies = fetch_studies(&study_names);
    println!("Typegen studies");
    gen_types(&studies, "studies");

    println!("Done");
}

// TODO: parallelize

fn gen_types(ns: &HashMap<&str, Value>, name: &str) {
    let v = ns.values().collect::<Vec<_>>();
    let Some((f, r)) = v.split_first() else {
        return;
    };
    let s = r.iter().fold(value_to_shape(f), |acc, c| {
        common_shape(acc, value_to_shape(c))
    });

    let pathstr = "./.cache/types/".to_owned() + name + ".rs";
    let path = Path::new(&pathstr);
    path.parent()
        .and_then(|parent| fs::create_dir_all(parent).ok());

    if let (_, Some(c)) = shape_to_code(name, &s) {
        fs::File::create(path)
            .and_then(|mut f| {
                f.write_all(("use serde::{Deserialize, Serialize};\n\n".to_owned() + &c).as_bytes())
            })
            .unwrap_or_else(|_| println!("Couldn't write {} types", name));
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
// 4462
fn parse_studies<'a>(modules: &'a HashMap<&str, Value>) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = modules
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
        .collect();

    res.sort();
    res.dedup();
    res
}

fn fetch_modules<'a>(module_names: &Vec<&'a str>) -> HashMap<&'a str, Value> {
    // huh it isn't even faster. bc of cloning?
    // let n_threads = 8;
    // let (tx, rx) = mpsc::channel::<HashMap<String, Value>>();
    //
    // for c in module_names.chunks(module_names.len() / n_threads + 1) {
    //     let tx = tx.clone();
    //     let co: Vec<String> = c.iter().map(|s| s.to_string()).collect();
    //     thread::spawn(move || {
    //         tx.send(
    //             co.iter()
    //                 .filter_map(|m| {
    //                     serde_json::from_str(&fetch_or_cache(
    //                         &(BASE_URL.to_owned() + "/allModules/" + m + ".json"),
    //                         &("modules/".to_owned() + m + ".json"),
    //                     ))
    //                     .ok()
    //                     .map(|v| (m.to_owned(), v))
    //                 })
    //                 .collect(),
    //         )
    //         .expect("send failed");
    //     });
    // }
    // drop(tx);
    //
    // let mut combined = HashMap::new();
    // for map in rx.iter() {
    //     combined.extend(map);
    // }
    // combined

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
// 288
fn parse_modules(html: &str) -> Vec<String> {
    html.match_indices("href=\"allModules/")
        .filter_map(|(i, _)| {
            let f = i + 17;
            let h = html.as_bytes();
            if f < h.len()
                && let Some(t) = &h[f..].iter().position(|&b| b == b'"')
                && let Ok(res) = from_utf8(&h[f..(t + f - 5)])
            {
                return Some(decode_html(res));
            }
            None
        })
        .collect()
}
