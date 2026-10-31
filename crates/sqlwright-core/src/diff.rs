use std::collections::HashSet;

use crate::{change::Change, snapshot::Snapshot};

pub(crate) fn diff<'a>(old: &'a Snapshot, new: &'a Snapshot) -> Vec<Change<'a>> {
    let mut changes = vec![];

    create_schema_diff(&mut changes, old, new);

    changes
}

fn create_schema_diff<'a>(changes: &mut Vec<Change<'a>>, old: &'a Snapshot, new: &'a Snapshot) {
    let old_tables_names: HashSet<&str> = old.tables.iter().map(|t| t.name.as_str()).collect();

    for t in &new.tables {
        if !old_tables_names.contains(t.name.as_str()) {
            changes.push(Change::CreateTable(t));
        }
    }
}
