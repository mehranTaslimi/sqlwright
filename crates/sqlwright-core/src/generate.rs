use std::path::Path;

use crate::{Backend, TableDef, snapshot::Snapshot};

pub fn generate<B: Backend>(
    _pool: &sqlx::Pool<B::Database>,
    root: &Path,
    out: &Path,
    tables: &[TableDef],
) {
    let snapshot = Snapshot::new(&root.to_path_buf());
    let old = snapshot.load();
    let new = snapshot.from(&tables);
    snapshot.save(&new);
}

fn diff_snapshot(root: &Path) {}
