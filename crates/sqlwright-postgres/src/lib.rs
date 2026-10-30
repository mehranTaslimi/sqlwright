use sqlwright_core::types::{BigInt, Bool, Double, Int, Real, SmallInt, Text};
use sqlwright_core::{Backend, impl_sql_types};

pub mod types;

#[derive(Debug, Clone, Copy, Default)]
pub struct Pg;

impl Backend for Pg {
    type Database = sqlx::Postgres;
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
