// yoinked from https://github.com/evestera/thesis

use std::collections::HashMap;

use serde_json::{Map, Value};

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Any,
    Bottom,
    Bool,
    StringS,
    Int,
    Float,
    List { elem_type: Box<Shape> },
    Recd { fields: HashMap<String, Shape> },
    Optional(Box<Shape>),
}

pub fn value_to_shape(value: &Value) -> Shape {
    match *value {
        Value::Null => Shape::Optional(Box::new(Shape::Bottom)),
        Value::Bool(_) => Shape::Bool,
        Value::Number(ref n) => {
            if n.is_i64() {
                Shape::Int
            } else {
                Shape::Float
            }
        }
        Value::String(_) => Shape::StringS,
        Value::Array(ref values) => array_to_shape(values),
        Value::Object(ref map) => object_to_shape(map),
    }
}

fn array_to_shape(values: &[Value]) -> Shape {
    let inner = values.iter().fold(Shape::Bottom, |shape, val| {
        let shape2 = value_to_shape(val);
        common_shape(shape, shape2)
    });
    Shape::List {
        elem_type: Box::new(inner),
    }
}

fn object_to_shape(map: &Map<String, Value>) -> Shape {
    let inner = map
        .iter()
        .map(|(name, value)| (name.clone(), value_to_shape(value)))
        .collect();
    Shape::Recd { fields: inner }
}

pub fn common_shape(a: Shape, b: Shape) -> Shape {
    if a == b {
        return a;
    }
    use self::Shape::*;
    match (a, b) {
        (a, Bottom) | (Bottom, a) => a,
        (Int, Float) | (Float, Int) => Float,
        (a, Optional(b)) | (Optional(b), a) => make_optional(common_shape(a, *b)),
        (List { elem_type: e1 }, List { elem_type: e2 }) => List {
            elem_type: Box::new(common_shape(*e1, *e2)),
        },
        (Recd { fields: f1 }, Recd { fields: f2 }) => Recd {
            fields: common_field_shapes(f1, f2),
        },
        _ => Any,
    }
}

fn make_optional(a: Shape) -> Shape {
    use self::Shape::*;
    match a {
        Any | Optional(_) => a,
        non_nullable => Optional(Box::new(non_nullable)),
    }
}

fn common_field_shapes(
    f1: HashMap<String, Shape>,
    mut f2: HashMap<String, Shape>,
) -> HashMap<String, Shape> {
    if f1 == f2 {
        return f1;
    }
    let mut unified = HashMap::new();
    for (key, val) in f1.into_iter() {
        match f2.remove(&key) {
            Some(val2) => {
                unified.insert(key, common_shape(val, val2));
            }
            None => {
                unified.insert(key, make_optional(val));
            }
        }
    }
    for (key, val) in f2.into_iter() {
        unified.insert(key, make_optional(val));
    }
    unified
}

pub fn shape_to_code(name: &str, shape: &Shape) -> (String, Option<String>) {
    match *shape {
        Shape::Any | Shape::Bottom => ("::serde_json::Value".into(), None),
        Shape::Bool => ("bool".into(), None),
        Shape::StringS => ("String".into(), None),
        Shape::Int => ("i64".into(), None),
        Shape::Float => ("f64".into(), None),
        Shape::List { elem_type: ref e } => {
            let (inner, inner_defs) = shape_to_code(name, e);
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
                let (field_type, field_defs) = shape_to_code(&fname, val);
                if let Some(defs) = field_defs {
                    inner_defs += &defs;
                }
                struct_def += &format!("  pub {}: {},\n", to_snake_case(key), field_type);
            }
            struct_def += &format!("}}\n");

            (type_name, Some(struct_def + &inner_defs))
        }
        Shape::Optional(ref e) => {
            let (inner, inner_defs) = shape_to_code(name, e);
            if **e == Shape::Bottom {
                (inner, inner_defs)
            } else {
                (format!("Option<{}>", inner), inner_defs)
            }
        }
    }
}

fn uppercase_first_letter(s: &str) -> String {
    match s.chars().next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + &s[1..],
    }
}

fn to_snake_case(s: &str) -> String {
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
