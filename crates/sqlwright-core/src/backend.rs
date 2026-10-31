use crate::{
    ColumnDef, TableDef,
    types::{Nullable, SqlType},
};

pub trait Backend: Sized + Send + Sync + 'static {
    type Database: sqlx::Database;

    fn quote_ident(name: &str) -> String {
        format!("\"{}\"", name)
    }

    fn create_tables(tables: &[TableDef]) -> String {
        tables
            .iter()
            .map(Self::create_table)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn create_table(table: &TableDef) -> String {
        format!(
            "create table if not exists {} (\n{}\n);",
            &Self::quote_ident(table.name),
            Self::create_columns(table.columns)
        )
    }

    fn create_columns(columns: &'static [ColumnDef]) -> String {
        columns
            .iter()
            .map(Self::create_column)
            .collect::<Vec<_>>()
            .join(",\n")
    }

    fn create_column(column: &'static ColumnDef) -> String {
        format!(
            "  {} {}{}",
            Self::quote_ident(column.name),
            column.sql_type,
            if column.nullable { "" } else { " not null" },
        )
    }
}

#[diagnostic::on_unimplemented(
    message = "the `{Self}` backend does not support the SQL type `{S}`",
    label = "`{S}` is not supported by `{Self}`",
    note = "a backend crate adds support with `impl HasSqlType<{S}> for {Self} {{}}`"
)]
pub trait HasSqlType<S: SqlType>: Backend {
    const TYPE_NAME: &'static str;
    const NULLABLE: bool = false;
}

impl<B, S> HasSqlType<Nullable<S>> for B
where
    B: HasSqlType<S>,
    S: SqlType,
{
    const TYPE_NAME: &'static str = <B as HasSqlType<S>>::TYPE_NAME;
    const NULLABLE: bool = true;
}
