use core::marker::PhantomData;

pub trait SqlType: 'static {}

crate::sql_types! {
    Bool,
    SmallInt,
    Int,
    BigInt,
    Real,
    Double,
    Text,
}

pub struct Nullable<S>(PhantomData<fn() -> S>);

impl<S: SqlType> SqlType for Nullable<S> {}
