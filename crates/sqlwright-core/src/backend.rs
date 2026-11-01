use crate::{
    change::Change,
    snapshot::TableSnapshot,
    types::{Nullable, SqlType},
};

pub trait Backend: Sized + Send + Sync + 'static {
    type Database: sqlx::Database;

    fn render(change: &Change<'_>) -> String {
        use Change::*;

        match change {
            CreateTable(t) => Self::create_table(t),
            _ => "".to_owned(),
        }
    }

    fn quote_ident(name: &str) -> String;
    fn create_table(table: &TableSnapshot) -> String;
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
