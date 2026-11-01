use sqlwright_core::types::{BigInt, Bool, Double, Int, Real, SmallInt, Text};
use sqlwright_core::{Backend, TableSnapshot, impl_sql_types};

pub mod types;

#[derive(Debug, Clone, Copy, Default)]
pub struct Pg;

impl Backend for Pg {
    type Database = sqlx::Postgres;

    fn quote_ident(name: &str) -> String {
        format!("\"{}\"", name)
    }

    fn create_table(table: &TableSnapshot) -> String {
        format!(
            "create table if not exists {} (\n{}\n);",
            Self::quote_ident(&table.name),
            table
                .columns
                .iter()
                .map(|column| {
                    format!(
                        "   {} {}{}",
                        Self::quote_ident(&column.name),
                        column.sql_type,
                        if column.nullable { "" } else { " not null" },
                    )
                })
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }
}

impl_sql_types!(Pg {
    Bool => "boolean",
    SmallInt => "smallint",
    Int => "integer",
    BigInt => "bigint",
    Real => "real",
    Double => "double precision",
    Text => "text",
});
