use sqlwright_core::types::{BigInt, Bool, Double, Int, Real, SmallInt, Text};
use sqlwright_core::{Backend, HasSqlType};
#[derive(Debug, Clone, Copy, Default)]
pub struct Pg;

impl Backend for Pg {
    type Database = sqlx::Postgres;
}

impl HasSqlType<Bool> for Pg {}
impl HasSqlType<SmallInt> for Pg {}
impl HasSqlType<Int> for Pg {}
impl HasSqlType<BigInt> for Pg {}
impl HasSqlType<Real> for Pg {}
impl HasSqlType<Double> for Pg {}
impl HasSqlType<Text> for Pg {}
