#[macro_export]
#[doc(hidden)]
macro_rules! impl_sql_types {
    ($backend:ty { $($sql:ty => $name:literal),* $(,)? }) => {
        $(
            impl $crate::HasSqlType<$sql> for $backend {
                const TYPE_NAME: &'static str = $name;
            }
        )*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! sql_types {
    ($( $name:ident ),* $(,)?) => {
        $(
            pub enum $name {}

            impl $crate::types::SqlType for $name {}
        )*
    };
    ($backend:ty { $( $name:ident => $sql:literal ),* $(,)? }) => {
        $crate::sql_types!($($name),*);
        $crate::impl_sql_types!($backend { $($name => $sql),* });
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! generate {
    { tables: [ $first:ty $(, $rest:ty)* $(,)? ], pool: $pool:expr, out: $out:expr $(,)? } => {
        $crate::generate::<<$first as $crate::Table>::Backend>(
            &$pool,
            ::std::path::Path::new(env!("CARGO_MANIFEST_DIR")),
            ::std::path::Path::new($out),
            &[
                <$first as $crate::Table>::DEF,
                $( <$rest as $crate::Table>::DEF ),*
            ],
        )
    };
    { tables: [ $($table:ty),+ $(,)? ], pool: $pool:expr $(,)? } => {
        $crate::generate! { tables: [ $($table),+ ], pool: $pool, out: "migrations" }
    };
    { tables: [ $($table:ty),+ $(,)? ], $pool:ident $(, $($rest:tt)*)? } => {{
        $crate::__expect_pool!($pool);
        $crate::generate! { tables: [ $($table),+ ], pool: $pool $(, $($rest)*)? }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __expect_pool {
    (pool) => {};
    ($other:ident) => {
        compile_error!(concat!(
            "expected `pool` or `pool: ",
            stringify!($other),
            "`"
        ))
    };
}
