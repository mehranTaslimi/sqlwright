use std::collections::HashSet;

use crate::{
    change::Change::{self, CreateTable, DropTable},
    snapshot::Snapshot,
};

pub(crate) fn diff<'a>(old: &'a Snapshot, new: &'a Snapshot) -> Vec<Change<'a>> {
    let mut changes = vec![];

    table_diff(&mut changes, old, new);

    changes
}

fn table_diff<'a>(changes: &mut Vec<Change<'a>>, old: &'a Snapshot, new: &'a Snapshot) {
    let old_names: HashSet<&str> = old.tables.iter().map(|t| t.name.as_str()).collect();
    let new_names: HashSet<&str> = new.tables.iter().map(|t| t.name.as_str()).collect();

    changes.extend(
        new.tables
            .iter()
            .filter(|t| !old_names.contains(t.name.as_str()))
            .map(CreateTable),
    );

    changes.extend(
        old.tables
            .iter()
            .filter(|t| !new_names.contains(t.name.as_str()))
            .map(|t| DropTable { name: &t.name }),
    );
}
