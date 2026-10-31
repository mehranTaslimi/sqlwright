use std::{fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::TableDef;

#[derive(Serialize, Deserialize, PartialEq)]
pub(crate) struct TableSnapshot {
    pub(crate) name: String,
    pub(crate) columns: Vec<ColumnSnapshot>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub(crate) struct ColumnSnapshot {
    pub(crate) name: String,
    pub(crate) sql_type: String,
    pub(crate) nullable: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub(crate) struct SnapshotContent {
    pub(crate) version: u32,
    pub(crate) tables: Vec<TableSnapshot>,
}

pub(crate) struct Snapshot {
    version: u32,
    root: PathBuf,
}

impl Snapshot {
    pub(crate) fn new(root: &PathBuf) -> Self {
        Self {
            version: 1,
            root: root.join(".sqlwright"),
        }
    }

    pub(crate) fn save(&self, content: &SnapshotContent) -> io::Result<()> {
        self.ensure_path()?;

        fs::write(
            self.root.join("snapshot.json"),
            serde_json::to_string_pretty(content)?,
        )?;

        Ok(())
    }

    pub(crate) fn from(&self, tables: &[TableDef]) -> SnapshotContent {
        SnapshotContent {
            version: self.version,
            tables: tables
                .iter()
                .map(|t| TableSnapshot {
                    name: t.name.to_string(),
                    columns: t
                        .columns
                        .iter()
                        .map(|c| ColumnSnapshot {
                            name: c.name.to_string(),
                            nullable: c.nullable.to_string(),
                            sql_type: c.sql_type.to_string(),
                        })
                        .collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>(),
        }
    }

    pub(crate) fn load(&self) {}
}

impl Snapshot {
    fn ensure_path(&self) -> io::Result<()> {
        if !self.root.exists() {
            fs::create_dir_all(&self.root)?;
        }

        Ok(())
    }
}
