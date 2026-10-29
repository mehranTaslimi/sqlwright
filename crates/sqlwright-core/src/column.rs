use core::fmt;
use core::marker::PhantomData;

use crate::backend::HasSqlType;
use crate::table::Table;
use crate::types::SqlType;

pub struct Column<T, S> {
    name: &'static str,
    _marker: PhantomData<fn() -> (T, S)>,
}

impl<T, S> Column<T, S>
where
    T: Table,
    S: SqlType,
    T::Backend: HasSqlType<S>,
{
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }
}

impl<T: Table, S> Column<T, S> {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn table_name(&self) -> &'static str {
        T::NAME
    }
}

impl<T, S> Clone for Column<T, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, S> Copy for Column<T, S> {}

impl<T: Table, S> fmt::Debug for Column<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", T::NAME, self.name)
    }
}
