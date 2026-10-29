use core::marker::PhantomData;

pub trait SqlType: 'static {}

macro_rules! sql_types {
    ($( $(#[$meta:meta])* $name:ident ),* $(,)?) => {
        $(
            $(#[$meta])*
            pub enum $name {}

            impl SqlType for $name {}
        )*
    };
}

sql_types! {
    /// `BOOLEAN`
    Bool,
    /// `SMALLINT` (16-bit integer)
    SmallInt,
    /// `INTEGER` (32-bit integer)
    Int,
    /// `BIGINT` (64-bit integer)
    BigInt,
    /// `REAL` (32-bit float)
    Real,
    /// `DOUBLE PRECISION` (64-bit float)
    Double,
    /// `TEXT`
    Text,
}

pub struct Nullable<S>(PhantomData<fn() -> S>);

impl<S: SqlType> SqlType for Nullable<S> {}
