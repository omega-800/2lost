use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde_json::{Map, Number, Value};

use crate::{
    fns::{read_bloat_into_mem, read_bloat_into_mem_untyped, to_snake_case},
    vars::{MODULES, MODULES_MAP, STUDIES, STUDIES_MAP},
};

pub fn populate_sql(studies: Vec<Value>, modules: Vec<Value>) -> String {
    let mut sm = HashMap::new();

    println!("Merging studies");
    for s in studies {
        unroll_value(Rc::new(RefCell::new(s)), STUDIES, &STUDIES_MAP, &mut sm, 1);
    }

    println!("Merging modules");
    for s in modules {
        // this definitely won't go wrong
        unroll_value(Rc::new(RefCell::new(s)), MODULES, &MODULES_MAP, &mut sm, 1);
    }

    let mut inserts = "".to_string();
    for (table, rows) in sm {
        for (id, row) in rows {
            inserts += &gen_sql_insert(
                &table,
                &row.borrow(),
                if table == MODULES {
                    &MODULES_MAP
                } else if table == STUDIES {
                    &STUDIES_MAP
                } else {
                    &[]
                },
            );
        }
    }

    inserts
}

fn unroll_value(
    value: Rc<RefCell<Value>>,
    name: &str,
    m: &[&str],
    res: &mut HashMap<String, HashMap<i64, Rc<RefCell<Value>>>>,
    // this scuffed id should only be necessary for modules_durchfuehrungen
    idd: i64,
) {
    let value_borrowed = value.borrow();
    match &*value_borrowed {
        Value::Array(values) => {
            let values_clone = values.clone();
            drop(value_borrowed);

            for v in values_clone {
                unroll_value(Rc::new(RefCell::new(v)), name, &[], res, idd);
            }
        }
        Value::Object(map) => {
            let id = get_id_int(map).unwrap_or(idd);
            let mut map_clone = map.clone();
            drop(value_borrowed);

            let entry = res.entry(name.to_string()).or_default();

            if let Some(prev) = entry.get(&id) {
                let mut prev_guard = prev.borrow_mut();
                let value_guard = value.borrow();
                merge_values(&mut prev_guard, &value_guard);
                drop(prev_guard);
                drop(value_guard);
            } else {
                map_clone.insert("id".to_string(), id.into());
                let nvalue = Value::Object(map_clone.clone());
                entry.insert(id, Rc::new(RefCell::new(nvalue)));
            }

            for (key, val) in &map_clone {
                let pos = m.iter().position(|kk| *kk == key);
                let mapped = pos.and_then(|i| m.get(i + 1)).copied().unwrap_or(key);
                unroll_value(Rc::new(RefCell::new(val.clone())), mapped, &[], res, id);
            }
        }
        _ => {}
    }
}

// TODO: macro?
// fn unroll_value<'a, 'b>(
//     value: &'a mut Value,
//     name: &'a str,
//     m: &[&'a str],
//     res: &'b mut HashMap<&'a str, HashMap<i64, &'a mut Value>>,
// ) {
//     // TODO: id's
//     match *value {
//         Value::Array(ref mut values) => {
//             for v in values.iter_mut() {
//                 unroll_value(v, name, &[], res);
//             }
//         }
//         Value::Object(ref mut map) => {
//             if let Some(id) = get_id_int(&map) {
//                 let entry = res.entry(name).or_default();
//                 if let Some(prev) = entry.get_mut(&id) {
//                     merge_values(*prev, value);
//                 } else {
//                     entry.insert(id, value);
//                 }
//
//                 for (key, val) in map {
//                     let pos = m.iter().position(|kk| **kk == *key);
//                     let mapped = pos.and_then(|i| m.get(i + 1)).copied().unwrap_or(key);
//                     unroll_value(val, mapped, &[], res);
//                 }
//             }
//         }
//         _ => {}
//     }
// }

fn b2s(b: &bool) -> Option<String> {
    Some(if *b { "1".to_string() } else { "0".to_string() })
}
fn n2s(n: &Number) -> Option<String> {
    if n.is_i64() {
        n.as_i64().map(|n| n.to_string())
    } else {
        n.as_f64().map(|n| n.to_string())
    }
}
fn s2s(s: &String) -> Option<String> {
    Some(format!("'{}'", s.replace("'", "\\'")))
}
fn get_id_int(o: &Map<String, Value>) -> Option<i64> {
    if let Some(id) = o.get("id")
        && let Value::Number(n) = id
    {
        n.as_i64()
    } else {
        eprint!("No id field / wrong id type");
        None
    }
}
fn get_id(o: &Map<String, Value>) -> Option<String> {
    if let Some(id) = o.get("id") {
        match id {
            Value::Bool(b) => b2s(b),
            Value::Number(n) => n2s(n),
            Value::String(s) => s2s(s),
            _ => {
                eprint!("Wrong id value");
                None
            }
        }
    } else {
        eprint!("No id field");
        None
    }
}

fn gen_sql_insert(parent: &str, value: &Value, m: &[&str]) -> String {
    let Value::Object(fields) = value else {
        return "".to_string();
    };

    let Some(parent_id) = get_id(fields) else {
        eprintln!("in {} skipping!", parent);
        return "".to_string();
    };

    let mut inames = vec![];
    let mut ivalues = vec![];
    let mut extra = "".to_string();

    for (k, v) in fields.iter() {
        let pos = m.iter().position(|kk| **kk == *k);
        let mapped = if let Some(i) = pos {
            *m.get(i + 1).unwrap(/* trust me bro */)
        } else {
            k
        };
        let (v, a) = match v {
            Value::Null => (None, None),
            Value::Bool(b) => (b2s(b), None),
            Value::Number(n) => (n2s(n), None),
            Value::String(s) => (s2s(s), None),
            Value::Array(a) => {
                let jname = format!("{}_{}", parent, k);
                let mut additional = "".to_string();
                for v in a {
                    if let Value::Object(v) = v {
                        if let Some(ref_id) = get_id(v) {
                            additional += &format!(
                                "INSERT INTO {} ({}, {}) VALUES ({}, {});\n",
                                jname, k, parent, ref_id, parent_id
                            );
                        } else {
                            eprintln!("in {} in referenced {} ({}), skipping!", parent, k, mapped);
                        }
                    } else {
                        eprintln!(
                            "List is not of type Objects in {} referencing {} ({}), skipping!",
                            parent, k, mapped
                        );
                    }
                }

                (Some(parent_id.clone()), Some(additional))
            }
            Value::Object(o) => match get_id(o) {
                Some(id) => (Some(id), None),
                None => {
                    eprintln!("in {} in referenced {} ({}), skipping!", parent, k, mapped);
                    (None, None)
                }
            },
        };

        if let Some(a) = a {
            extra += &a;
        }
        if let Some(v) = v {
            inames.push(to_snake_case(k.as_ref()));
            ivalues.push(v);
        }
    }
    format!(
        "INSERT INTO {} ({}) VALUES ({});\n{}",
        parent,
        inames.join(", "),
        ivalues.join(", "),
        extra
    )
}

fn merge_values(this: &mut Value, other: &Value) -> bool {
    match (this, other) {
        (&mut Value::Object(ref mut res), Value::Object(other)) => {
            for k in other.keys() {
                res.entry(k.clone()).or_insert(Value::Null);
            }
            true
        }
        (&mut Value::Array(ref mut this), Value::Array(other)) => {
            this.extend(other.clone());
            false
        }
        (_, &Value::Null) => false,
        (this @ &mut Value::Null, other) => {
            *this = other.clone();
            false
        }
        _ => false,
    }
}
