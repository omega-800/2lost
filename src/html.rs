use std::{
    fs::{self, File},
    io::BufReader,
};

use crate::{
    fns::{create_write_file, to_alphanum},
    types::{Modules, Studies},
    vars::{CACHE_PATH, HTML_PATH, MODULES_PATH, STUDIES_PATH},
};

// - orte
// - vorausg_kenntnisse
// - methoden
// - durchfuehrungen
// - mengen
// - zuordnungen
// - zustand
// - semester_bewertung
// - voraussetzungen
// - empfehlungen
// - kreditpunkte
const MODULE_FILTERS: [&str; 12] = [
    "sprache",
    "orte",
    "vorausg_kenntnisse",
    "methoden",
    "durchfuehrungen",
    "mengen",
    "zuordnungen",
    "zustand",
    "semester_bewertung",
    "voraussetzungen",
    "empfehlungen",
    "kreditpunkte",
];

pub fn gen_html() {
    println!("Reading bloat into memory");
    let (studies, modules) = read_bloat_into_mem();

    println!("Generating studies.html");
    let studies_html = studies_to_html(&studies);

    create_write_file(
        &(CACHE_PATH.to_owned() + HTML_PATH + "studies.html"),
        &mkbody("", &studies_html),
    )
    .unwrap_or_else(|_| println!("Couldn't write studies.html"));
    println!("Wrote studies.html");

    println!("Generating modules.html");
    let modules_html = modules_to_html(&modules);
    let (mf_css, mf_html) = genfilter(&modules);
    create_write_file(
        &(CACHE_PATH.to_owned() + HTML_PATH + "modules.html"),
        &mkbody(&mf_css, &(mf_html + &modules_html)),
    )
    .unwrap_or_else(|_| println!("Couldn't write modules.html"));
    println!("Wrote modules.html");
}

impl Modules {
    pub fn get_field(&self, field: &str) -> Vec<&str> {
        match field {
            "sprache" => vec![&self.sprache],
            "orte" => self
                .orte
                .iter()
                .flat_map(|os| os.iter().map(|o| o.ort.as_str()).collect::<Vec<_>>())
                .collect(),
            // "vorausg_kenntnisse" => self
            //     .vorausg_kenntnisse
            //     .iter()
            //     .map(String::as_str)
            //     .collect::<Vec<_>>(),
            "methoden" => self.methoden.iter().map(String::as_str).collect::<Vec<_>>(),
            "durchfuehrungen" => self
                .durchfuehrungen
                .iter()
                .flat_map(|d| {
                    vec![
                        // TODO:
                        d.end_semester.as_str(),
                    ]
                })
                .collect::<Vec<_>>(),
            "mengen" => self
                .mengen
                .iter()
                .flat_map(|os| os.iter().map(|o| o.name.as_str()).collect::<Vec<_>>())
                .collect(),
            "zuordnungen" => self
                .zuordnungen
                .iter()
                .flat_map(|os| {
                    // TODO:
                    os.iter()
                        .map(|o| o.bezeichnung.as_str())
                        .collect::<Vec<_>>()
                })
                .collect(),
            "zustand" => vec![&self.zustand],
            "semester_bewertung" => vec![&self.semester_bewertung],
            // "voraussetzungen" => &self.voraussetzungen,
            // "empfehlungen" => &self.empfehlungen,
            // "kreditpunkte" => {
            //     self.kreditpunkte
            //         .iter()
            //         .map(|k| k.to_string())
            //         .collect::<Vec<_>>();
            // }
            _ => vec![],
        }
    }
}

fn genfilter(modules: &[Modules]) -> (String, String) {
    let mut css = "".to_string();
    let mut html = "<h1>Filter</h1>".to_string();
    for filter in MODULE_FILTERS {
        let mut smb: Vec<_> = modules
            .iter()
            .flat_map(|m| m.get_field(filter))
            .map(to_alphanum)
            .collect();
        smb.sort();
        smb.dedup();
        if smb.is_empty() {
            continue;
        }

        css += &smb
            .iter()
            .map(|i| {
                format!(
                    r#"
                    #filter_{}_{}:checked ~ .items .item_{}_{} {{
                        display: block; 
                    }}
                    "#,
                    filter, i, filter, i
                )
            })
            .collect::<String>();
        // "<div class=\"filter-items\">".to_string() + &
        html += "<h2>";
        html += filter;
        html += "</h2>";
        html += &smb
            .iter()
            .map(|i| {
                format!(
                    r#"
                    <input type="checkbox" id="filter_{}_{}" class="filter-item" />
                    <label for="filter_{}_{}">{}</label>
                    "#,
                    filter, i, filter, i, i
                )
            })
            .collect::<String>();
        // + "</div>",
    }
    (css, html)
}

fn mkbody(css: &str, body: &str) -> String {
    r#"<html>
    <style>
        .items {
            display: grid;
            grid-template-columns: 25% 25% 25% 25%;
        }
        .item {
            padding: 10px;
            margin: 10px;
            background-color: #f8f8f8;
        }
        .hidden, .item {
            display: none;
        }"#
    .to_string()
        + css
        + "</style><body>"
        + body
        + "</body></html>"
}

fn modules_to_html(modules: &[Modules]) -> String {
    "<div class=\"items modules\">".to_string()
        + &modules
            .iter()
            .map(|m| {
                let filter_classes = MODULE_FILTERS
                    .iter()
                    .flat_map(|filter| {
                        m.get_field(filter)
                            .into_iter()
                            .map(|f| "item_".to_string() + filter + "_" + &to_alphanum(f))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(
                    r#"
                    <div class="module item {}">
                        <h1>{}</h1>
                        <p>{}</p>
                        <p>{}</p>
                    </div>
                    "#,
                    filter_classes, m.bezeichnung, m.kuerzel, m.semester_bewertung
                )
            })
            .collect::<String>()
        + "</div>"
}

fn studies_to_html(studies: &[Studies]) -> String {
    "<div class=\"items studies\">".to_string()
        + &studies
            .iter()
            .map(|s| {
                format!(
                    r#"
                    <div class="study item">
                        <h1>{}</h1>
                        <p>{}</p>
                        <p>{}</p>
                    </div>
                    "#,
                    s.kuerzel, s.bezeichnung, s.version_kuerzel,
                )
            })
            .collect::<String>()
        + "</div>"
}

// don't mind the duplicated logic

fn read_bloat_into_mem() -> (Vec<Studies>, Vec<Modules>) {
    (pawse(STUDIES_PATH), pawse(MODULES_PATH))
}

fn pawse<T: serde::de::DeserializeOwned>(path: &str) -> Vec<T> {
    fs::read_dir(CACHE_PATH.to_owned() + path)
        .unwrap_or_else(|_| panic!("Cached path doesn't exist: {}", path))
        .filter_map(|s| {
            s.ok().and_then(|s| {
                File::open(s.path())
                    .ok()
                    .and_then(|f| serde_json::from_reader(BufReader::new(f)).ok())
            })
        })
        .collect()
}
