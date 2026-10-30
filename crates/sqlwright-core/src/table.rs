use crate::{backend::Backend, schema::TableDef};

pub trait Table: Copy + Send + Sync + 'static {
    type Backend: Backend;
    const DEF: TableDef;
}
