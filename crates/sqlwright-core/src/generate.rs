use std::{io, path::Path};

use crate::{Backend, TableDef, diff::diff, snapshot::Snapshot};

pub fn generate<B: Backend>(
    _pool: &sqlx::Pool<B::Database>,
    root: &Path,
    out: &Path,
    tables: &[TableDef],
) -> io::Result<()> {
    let old = Snapshot::load_or_init(root)?;
    let new = Snapshot::from(tables);

    let mut sql = vec![];

    for change in diff(&old, &new) {
        sql.push(B::render(&change));
    }

    println!("{:#?}", sql);

    Ok(())
}
