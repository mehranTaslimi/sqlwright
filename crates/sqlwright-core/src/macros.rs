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

            impl $crate::SqlType for $name {}
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
    { $first:ty $(, $rest:ty)* $(,)? } => {
        $crate::generate::<<$first as $crate::Table>::Backend>(&[
            <$first as $crate::Table>::DEF,
            $( <$rest as $crate::Table>::DEF ),*
        ])
    };
}
