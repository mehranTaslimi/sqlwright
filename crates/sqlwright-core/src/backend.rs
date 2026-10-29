use crate::types::{Nullable, SqlType};

pub trait Backend: Sized + Send + Sync + 'static {
    type Database: sqlx::Database;
}

#[diagnostic::on_unimplemented(
    message = "the `{Self}` backend does not support the SQL type `{S}`",
    label = "`{S}` is not supported by `{Self}`",
    note = "a backend crate adds support with `impl HasSqlType<{S}> for {Self} {{}}`"
)]
pub trait HasSqlType<S: SqlType>: Backend {}

impl<B, S> HasSqlType<Nullable<S>> for B
where
    B: HasSqlType<S>,
    S: SqlType,
{
}
