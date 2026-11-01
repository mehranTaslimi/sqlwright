use std::{io, path::Path};

use crate::{Backend, TableDef, diff::diff, migration, snapshot::Snapshot};

pub fn generate<B: Backend>(
    _pool: &sqlx::Pool<B::Database>,
    root: &Path,
    out: &Path,
    tables: &[TableDef],
) -> io::Result<()> {
    let old = Snapshot::load_or_init(root)?;
    let new = Snapshot::from(tables);

    let sql = diff(&old, &new).iter().map(B::render).collect::<Vec<_>>();

    if sql.len() == 0 {
        return Ok(());
    }

    migration::write(out, "file", &sql.join("\n\n"))?;
    new.save(root)?;

    Ok(())
}
