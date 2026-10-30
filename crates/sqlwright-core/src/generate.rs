use crate::{Backend, TableDef};

pub fn generate<B: Backend>(tables: &[TableDef]) {
    println!(
        "{}",
        tables
            .iter()
            .map(B::create_table)
            .collect::<Vec<_>>()
            .join("\n")
    )
}
