use std::collections::HashMap;

use serde_json::Value;

use crate::{
    fns::{create_write_file, read_bloat_into_mem_untyped, to_snake_case, uppercase_first_letter},
    infer_types::{Shape, common_shape, value_to_shape},
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

pub fn gen_sql() {
    println!("Reading bloat into memory");
    let (studies, modules) = read_bloat_into_mem_untyped();

    if let Some(study_shape) = values_to_shape(&studies) {
        println!("Generating studies");
        // let study_shape = map_recd_tlks(study_shape, &STUDIES_MAP);
        gen_sql_types_for(&study_shape, STUDIES, &STUDIES_MAP);
    }

    if let Some(module_shape) = values_to_shape(&modules) {
        println!("Generating modules");
        // let module_shape = map_recd_tlks(module_shape, &MODULES_MAP);
        gen_sql_types_for(&module_shape, MODULES, &MODULES_MAP);
    }

    println!("Done");
}

pub fn gen_types() {
    println!("Reading bloat into memory");
    let (studies, modules) = read_bloat_into_mem_untyped();

    if let Some(study_shape) = values_to_shape(&studies) {
        println!("Generating studies");
        gen_rs_types_for(&study_shape, STUDIES);
    }

    if let Some(module_shape) = values_to_shape(&modules) {
        println!("Generating modules");
        gen_rs_types_for(&module_shape, MODULES);
    }

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

fn flatten_shape<'a>(
    shape: &'a Shape,
    name: &'a str,
    mut res: &'a mut HashMap<&'a str, Vec<&'a Shape>>,
    m: &[&str],
) -> &'a mut HashMap<&'a str, Vec<&'a Shape>> {
    match *shape {
        Shape::List { ref elem_type } => flatten_shape(elem_type, name, res, &[]),
        Shape::Recd { ref fields } => {
            res.entry(name).or_default().push(shape);
            for (k, v) in fields.iter() {
                res = flatten_shape(v, k, res, &[]);
            }
            res
        }
        Shape::Optional(ref shape) => flatten_shape(shape, name, res, m),
        _ => res,
    }
}

#[allow(dead_code)]
fn map_recd_tlks(shape: Shape, m: &[&str]) -> Shape {
    if let Shape::Recd { fields } = shape {
        return Shape::Recd {
            fields: fields
                .into_iter()
                .map(|(k, v)| {
                    let pos = m.iter().position(|kk| **kk == k);
                    (
                        if let Some(i) = pos {
                            (**m.get(i + 1).unwrap(/* trust me bro */)).to_string()
                        } else {
                            k
                        },
                        v,
                    )
                })
                .collect(),
        };
    };
    shape
}

fn gen_sql_types_for(v: &Shape, name: &str, m: &[&str]) {
    if let (_, Some(c)) = shape_to_sql(name, name, v, m) {
        create_write_file(&(CACHE_PATH.to_owned() + TYPES_PATH + name + ".sql"), &c)
            .unwrap_or_else(|_| println!("Couldn't write {} sql definition", name));
    }
}

fn shape_to_sql(parent: &str, name: &str, shape: &Shape, m: &[&str]) -> (String, Option<String>) {
    match *shape {
        Shape::Any | Shape::Bottom => ("".into(), None),
        Shape::Bool | Shape::Int => (
            "INTEGER ".to_string()
                + if name == "id" {
                    "PRIMARY KEY"
                } else {
                    "NOT NULL"
                },
            None,
        ),
        Shape::Float => ("REAL NOT NULL".into(), None),
        Shape::StringS => ("TEXT NOT NULL".into(), None),
        Shape::List { ref elem_type } => {
            let (inner, inner_defs) = shape_to_sql(parent, name, elem_type, &[]);

            let jname = format!("{}_{}", parent, inner);
            let jtable = format!(
                r#"CREATE TABLE {} (
  id INTEGER PRIMARY KEY REFERENCES {}(id),
  {} REFERENCES {}(id)
);
"#,
                jname, inner, parent, parent
            );
            // TODO:
            (
                jname,
                Some(inner_defs.map_or(jtable.clone(), |s| s + &jtable)),
            )
        }
        Shape::Recd { ref fields } => {
            // TODO:
            let type_name = to_snake_case(name);
            let mut inner_defs = String::new();

            let mut table_def = format!("CREATE TABLE {} (\n", type_name);
            for (key, val) in fields.iter() {
                let pos = m.iter().position(|k| **k == **key);
                let mapped = if let Some(i) = pos {
                    *m.get(i + 1).unwrap(/* trust me bro */)
                } else {
                    key
                };
                let (mut field_type, field_defs) = shape_to_sql(&type_name, mapped, val, &[]);
                if let Some(defs) = field_defs {
                    field_type = format!("REFERENCES {}(id)", field_type);
                    inner_defs += &defs;
                }
                table_def += &format!("  {} {},\n", to_snake_case(key), field_type);
            }
            if fields.iter().all(|(f, _)| f != "id") {
                table_def += "  id INTEGER PRIMARY KEY,\n";
            }
            table_def += ");\n";

            (type_name, Some(table_def + &inner_defs))
        }
        Shape::Optional(ref shape) => {
            let (inner, t) = shape_to_sql(parent, name, shape, m);
            if **shape == Shape::Bottom {
                (inner, t)
            } else {
                // TODO: remove NOT NULL
                (inner, t)
            }
        }
    }
}
