use crate::fns::{read_bloat_into_mem, read_bloat_into_mem_untyped};

pub fn populate_sql() {
    println!("Reading bloat into memory");
    let (studies, modules) = read_bloat_into_mem_untyped();
}
