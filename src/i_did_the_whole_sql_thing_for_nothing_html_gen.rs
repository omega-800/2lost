use std::collections::HashMap;

use serde_json::Value;

use crate::{
    fns::{create_write_file, read_bloat_into_mem},
    types::Module,
    vars::{CACHE_PATH, HTML_PATH},
};

enum FilterVal {
    Range(&'static str, &'static str),
    Simple(&'static str),
}
type FilterElem = (&'static str, Option<&'static str>, FilterVal);

const MODULE_FILTERS: [FilterElem; 14] = {
    use self::FilterVal::*;
    [
        ("orte", Some("/orte/bezeichnung"), Simple("/orte/id")),
        ("kreditpunkte", None, Simple("/kreditpunkte")),
        (
            "jahr",
            None,
            Range("/durchfuehrungen/begin_jahr", "/durchfuehrungen/end_jahr"),
        ),
        (
            "semester",
            None,
            Range(
                "/durchfuehrungen/begin_semester",
                "/durchfuehrungen/end_semester",
            ),
        ),
        (
            "studiengang",
            Some("/zuordnungen/bezeichnung"),
            Simple("/zuordnungen/kuerzel"),
        ),
        (
            "ist_abschluss_arbeit",
            None,
            Simple("/zuordnungen/ist_abschluss_arbeit"),
        ),
        (
            "sem_empfehlung",
            None,
            Simple("/zuordnungen/sem_empfehlung"),
        ),
        (
            "ist_pflichtmodul",
            None,
            Simple("/zuordnungen/ist_pflichtmodul"),
        ),
        (
            "voraussetzungen",
            Some("/voraussetzungen/bezeichnung"),
            Simple("/voraussetzungen/kuerzel"),
        ),
        ("zustand", None, Simple("/zustand")),
        ("semester_bewertung", None, Simple("/semester_bewertung")),
        (
            "vorgaenger",
            Some("/vorgaenger/bezeichnung"),
            Simple("/vorgaenger/kuerzel"),
        ),
        ("sprache", None, Simple("/sprache")),
        (
            "nachfolger",
            Some("/nachfolger/bezeichnung"),
            Simple("/nachfolger/kuerzel"),
        ),
    ]
    // mengen?
    // vorausg_kenntnisse=
    // TODO:
    // pruefung.dauer_prf_mue & pruefung_mue
    // pruefung.dauer_prf_schr & pruefung_schr
    // pruefung.zulassung
    // pruefung.anmeldbar
    // pruefung.art
    // kurse
    // anschlussmodule
    // dozenten
    // empfehlungen
};

pub fn gen_html() {
    println!("Reading bloat into memory");
    let (_, modules) = read_bloat_into_mem::<Value, Value>();

    println!("Generating modules.html");
    let modules_html = modules
        .iter()
        .map(module_to_html)
        .collect::<Vec<String>>()
        .join("\n");

    create_write_file(
        &(CACHE_PATH.to_owned() + HTML_PATH + "modules2.html"),
        &mkhtml(&(gen_filter_html(&modules) + &modules_html)),
    )
    .unwrap_or_else(|_| println!("Couldn't write modules2.html"));
}

fn mkhtml(body: &str) -> String {
    format!(
        r#"
<html>
<body>
    {}
</body>
</html>
"#,
        body
    )
}

fn get_string(v: &Value, k: &str) -> String {
    v.get(k).map_or_else(
        || "".to_string(),
        |v| {
            match v {
                Value::String(v) => v,
                _ => "",
            }
            .to_string()
        },
    )
}

fn gen_filter_html(ms: &[Value]) -> String {
    MODULE_FILTERS
        .iter()
        .map(|(n, d, v)| match v {
            FilterVal::Range(f, t) => {

                // TODO:
                // todo!()
                "".to_string()
            }
            FilterVal::Simple(p) => {
                let mut mapped = ms
                    .iter()
                    .filter_map(|m| {
                        m.pointer(p)
                            .and_then(Value::as_str)
                            .map(|val| (d.and_then(|d| m.pointer(d).and_then(|d| d.as_str())), val))
                    })
                    .collect::<Vec<_>>();
                mapped.sort();
                mapped.dedup();
                format!(
                    r#"
                    <label for="{}">{}</label>
                    <select id="{}" name="{}">
                        {}
                    </select>
                "#,
                    n,
                    n,
                    n,
                    n,
                    mapped
                        .iter()
                        .map(|(n, v)| {
                            format!(r#"<option value="{}">{}</option>"#, v, n.unwrap_or(v))
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn module_to_html(m: &Value) -> String {
    format!(
        r#"
        <div class="module item" {}>
            <h1>{}</h1>
            <p>{}</p>
            <p>{}</p>
        </div>
        "#,
        value_attrs(m, MODULE_FILTERS),
        get_string(m, "bezeichnung"),
        get_string(m, "kuerzel"),
        get_string(m, "semester_bewertung"),
    )
}

fn value_attrs(value: &Value, paths: [FilterElem; 14]) -> String {
    paths
        .iter()
        .map(|(k, _, p)| match p {
            FilterVal::Range(f, t) => {
                if let Some(fv) = value.pointer(f)
                    && *fv != Value::Null
                    && let Some(tv) = value.pointer(t)
                    && *tv != Value::Null
                {
                    format!("data-to-{}='{}' data-from-{}='{}'", k, tv, k, fv)
                } else {
                    "".to_string()
                }
            }
            FilterVal::Simple(v) => {
                if let Some(v) = value.pointer(v)
                    && *v != Value::Null
                {
                    format!("data-{}='{}'", k, v)
                } else {
                    "".to_string()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
