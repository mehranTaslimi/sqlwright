use sqlwright_core::types::{BigInt, Bool, Double, Int, Real, SmallInt, Text};
use sqlwright_core::{Backend, ColumnSnapshot, TableSnapshot, impl_sql_types};

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
            table.name,
            Self::create_columns(&table.columns)
        )
    }

    fn create_columns(columns: &[ColumnSnapshot]) -> String {
        columns
            .iter()
            .map(Self::create_column)
            .collect::<Vec<_>>()
            .join(",\n")
    }

    fn create_column(column: &ColumnSnapshot) -> String {
        format!(
            "  {} {}{}",
            Self::quote_ident(&column.name),
            column.sql_type,
            if column.nullable { "" } else { " not null" },
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
