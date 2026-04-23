use std::collections::HashMap;

use serde_json::Value;

use crate::{
    fns::{create_write_file, read_bloat_into_mem_untyped, to_snake_case, uppercase_first_letter},
    infer_types::{Shape, common_shape, reduce_to_common_shape, value_to_shape},
    vars::{CACHE_PATH, MODULES, STUDIES, TYPES_PATH},
};

const STUDIES_MAP: [&str; 6] = [
    "spezialisierungen",
    "studien",
    "parent",
    "studien",
    "studienberater",
    "dozenten",
];
const MODULES_MAP: [&str; 10] = [
    "nachfolger",
    "module",
    "vorgaenger",
    "module",
    "voraussetzungen",
    "module",
    "empfehlungen",
    "module",
    "anschlussmodule",
    "module",
];

fn do_the_boring_stuff() -> (Vec<Value>, Vec<Value>, Shape, Shape, HashMap<&str, Shape>) {
    println!("Reading bloat into memory");
    let (studies, modules) = read_bloat_into_mem_untyped();

    let mut all = HashMap::new();

    println!("Generating studies");
    let Some(study_shape) = values_to_shape(&studies) else {
        panic!("Failed inferring studies types");
    };
    let sh = unroll_shape(&study_shape, STUDIES, &STUDIES_MAP);

    println!("Generating modules");
    let Some(module_shape) = values_to_shape(&modules) else {
        panic!("Failed inferring modules types");
    };
    let mh = unroll_shape(&module_shape, MODULES, &MODULES_MAP);

    println!("Merging types");
    for (k, ss) in sh.iter().chain(mh.iter()) {
        if let Some(ss) = reduce_to_common_shape(ss) {
            if all.contains_key(k) {
                let prev = all.remove(k).unwrap();
                all.insert(*k, common_shape(prev, ss));
            } else {
                all.insert(*k, ss);
            }
        }
    }

    (studies, modules, study_shape, module_shape, all)
}

// TODO: move sql stuff to sql module
pub fn gen_sql() {
    let (studies, modules, study_shape, module_shape, all) = do_the_boring_stuff();

    println!("Codegen");
    let sql = all
        .iter()
        .map(|(k, v)| {
            gen_sql_table(
                k,
                v,
                &([
                    (&STUDIES_MAP as &[&'static str]),
                    (&MODULES_MAP as &[&'static str]),
                ]
                .concat()),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    create_write_file(&(CACHE_PATH.to_owned() + TYPES_PATH + "db.sql"), &sql)
        .unwrap_or_else(|_| println!("Couldn't write sql definition"));

    println!("Done");
}

pub fn gen_types() {
    let (studies, modules, study_shape, module_shape, all) = do_the_boring_stuff();

    println!("Codegen (json)");
    let (_, Some(study_rs)) = shape_to_rs(STUDIES, &study_shape) else {
        panic!("Failed generating study rs type");
    };
    let (_, Some(module_rs)) = shape_to_rs(MODULES, &module_shape) else {
        panic!("Failed generating module rs type");
    };

    create_write_file(
        &(CACHE_PATH.to_owned() + TYPES_PATH + "types_json.rs"),
        &("use serde::{Deserialize, Serialize};\n\n".to_owned() + &study_rs + &module_rs),
    )
    .unwrap_or_else(|_| println!("Couldn't write rs json types"));

    // TODO: extrapolate duplicated code
    println!("Codegen (sql)");

    let sql = all
        .iter()
        .map(|(k, v)| {
            gen_sql_rs(
                k,
                v,
                &([
                    (&STUDIES_MAP as &[&'static str]),
                    (&MODULES_MAP as &[&'static str]),
                ]
                .concat()),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    create_write_file(&(CACHE_PATH.to_owned() + TYPES_PATH + "types_sql.rs"), &sql)
        .unwrap_or_else(|_| println!("Couldn't write rs sql types"));

    println!("Done");
}

fn values_to_shape(v: &[Value]) -> Option<Shape> {
    let (f, r) = v.split_first()?;
    Some(r.iter().fold(value_to_shape(f), |acc, c| {
        common_shape(acc, value_to_shape(c))
    }))
}

fn gen_rs_types_for(v: &Shape, name: &str) {
    if let (_, Some(c)) = shape_to_rs(name, v) {
        create_write_file(
            &(CACHE_PATH.to_owned() + TYPES_PATH + name + ".rs"),
            &("use serde::{Deserialize, Serialize};\n\n".to_owned() + &c),
        )
        .unwrap_or_else(|_| println!("Couldn't write {} rs types", name));
    }
}

fn shape_to_rs(name: &str, shape: &Shape) -> (String, Option<String>) {
    match *shape {
        Shape::Any | Shape::Bottom => ("::serde_json::Value".into(), None),
        Shape::Bool => ("bool".into(), None),
        Shape::StringS => ("String".into(), None),
        Shape::Int => ("i64".into(), None),
        Shape::Float => ("f64".into(), None),
        Shape::List { elem_type: ref e } => {
            let (inner, inner_defs) = shape_to_rs(name, e);
            (format!("Vec<{}>", inner), inner_defs)
        }
        Shape::Recd { fields: ref map } => {
            let type_name = uppercase_first_letter(name);
            let mut inner_defs = String::new();

            let mut struct_def = format!(
                "#[derive(Serialize, Deserialize, Debug)]\n#[serde(rename_all = \"camelCase\")]\npub struct {} {{\n",
                type_name
            );
            for (key, val) in map.iter() {
                let fname = name.to_owned() + &uppercase_first_letter(key);
                let (field_type, field_defs) = shape_to_rs(&fname, val);
                if let Some(defs) = field_defs {
                    inner_defs += &defs;
                }
                struct_def += &format!("  pub {}: {},\n", to_snake_case(key), field_type);
            }
            struct_def += "}}\n";

            (type_name, Some(struct_def + &inner_defs))
        }
        Shape::Optional(ref e) => {
            let (inner, inner_defs) = shape_to_rs(name, e);
            if **e == Shape::Bottom {
                (inner, inner_defs)
            } else {
                (format!("Option<{}>", inner), inner_defs)
            }
        }
    }
}

fn unroll_shape<'a>(
    shape: &'a Shape,
    name: &'a str,
    m: &[&'a str],
) -> HashMap<&'a str, Vec<&'a Shape>> {
    let mut res: HashMap<&str, Vec<&Shape>> = HashMap::new();
    match *shape {
        Shape::List { ref elem_type } => unroll_shape(elem_type, name, &[]),
        Shape::Recd { ref fields } => {
            res.entry(name).or_default().push(shape);
            for (key, val) in fields.iter() {
                let pos = m.iter().position(|kk| **kk == *key);
                let mapped = if let Some(i) = pos {
                    *m.get(i + 1).unwrap(/* trust me bro */)
                } else {
                    key
                };
                for (key, val) in unroll_shape(val, mapped, &[]).iter() {
                    res.entry(key).or_default().extend(val);
                }
            }
            res
        }
        Shape::Optional(ref shape) => unroll_shape(shape, name, &[]),
        _ => res,
    }
}

// TODO: clean this up
pub fn gen_sql_table(parent: &str, shape: &Shape, m: &[&str]) -> String {
    let Shape::Recd { fields } = shape else {
        return "".to_string();
    };
    let gen_field = |field: &Shape, name: &str| -> (String, Option<String>) {
        let pos = m.iter().position(|k| **k == *name);
        let mapped = if let Some(i) = pos {
            *m.get(i + 1).unwrap(/* trust me bro */)
        } else {
            name
        };

        let (s, o) = match *field {
            Shape::Bool | Shape::Int => (
                "INTEGER ".to_string() + if name == "id" { "PRIMARY KEY" } else { "" },
                None,
            ),
            Shape::Float => ("REAL".to_string(), None),
            Shape::StringS => ("TEXT".to_string(), None),
            Shape::List { .. } => {
                let jname = format!("{}_{}", parent, name);
                // TODO: wrong, id should be name
                (
                    format!("REFERENCES {}(id)", jname),
                    Some(format!(
                        r#"
CREATE TABLE {} (
  id INTEGER PRIMARY KEY REFERENCES {}(id),
  {} REFERENCES {}(id)
);
"#,
                        jname, mapped, parent, parent
                    )),
                )
            }
            Shape::Recd { .. } => (format!("REFERENCES {}(id)", mapped), None),
            _ => ("".to_string(), None),
        };
        (format!("  {} {}", to_snake_case(name), s), o)
    };
    let mut additional = "".to_string();
    let mut table_def = format!("CREATE TABLE {} (\n", parent);
    let mut it = fields.iter().peekable();
    if fields.iter().all(|(f, _)| f != "id") {
        table_def += "  id INTEGER PRIMARY KEY,\n";
    }
    while let Some((k, field)) = it.next() {
        let (s, o) = match *field {
            Shape::Optional(ref f) => gen_field(f, k),
            _ => {
                let (a, b) = gen_field(field, k);
                (a + " NOT NULL", b)
            }
        };
        table_def += &s;
        if it.peek().is_some() {
            table_def += ",\n";
        }
        if let Some(o) = o {
            additional += &o;
        }
    }
    table_def + ");\n" + &additional
}

fn gen_sql_rs(parent: &str, shape: &Shape, m: &[&str]) -> String {
    todo!()
}
