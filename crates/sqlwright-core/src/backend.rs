use crate::{
    ColumnDef, TableDef,
    types::{Nullable, SqlType},
};

pub trait Backend: Sized + Send + Sync + 'static {
    type Database: sqlx::Database;

    fn quote_ident(name: &str) -> String {
        format!("\"{}\"", name)
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
            .map(|c| {
                format!(
                    "  {} {}{}",
                    Self::quote_ident(c.name),
                    c.sql_type,
                    if c.nullable { "" } else { " not null" },
                )
            })
            .collect::<Vec<_>>()
            .join(",\n")
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
