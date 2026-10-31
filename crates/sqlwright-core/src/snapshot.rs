use std::{
    fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{ColumnDef, TableDef};

const VERSION: u32 = 1;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Snapshot {
    pub(crate) version: u32,
    pub(crate) tables: Vec<TableSnapshot>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TableSnapshot {
    pub name: String,
    pub columns: Vec<ColumnSnapshot>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnSnapshot {
    pub name: String,
    pub sql_type: String,
    pub nullable: bool,
}

impl Snapshot {
    pub(crate) fn load_or_init(root: &Path) -> io::Result<Self> {
        match fs::read_to_string(path(root)) {
            Ok(json) => Ok(serde_json::from_str(&json)?),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let snapshot = Self::default();
                snapshot.save(root)?;
                Ok(snapshot)
            }
            Err(e) => Err(e),
        }
    }

    pub(crate) fn save(&self, root: &Path) -> io::Result<()> {
        let path = path(root);

        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }

        fs::write(path, serde_json::to_string_pretty(self)?)
    }
}

impl Default for Snapshot {
    fn default() -> Self {
        Self {
            version: VERSION,
            tables: Vec::new(),
        }
    }
}

impl From<&[TableDef]> for Snapshot {
    fn from(tables: &[TableDef]) -> Self {
        Self {
            version: VERSION,
            tables: tables.iter().map(TableSnapshot::from).collect(),
        }
    }
}

impl From<&TableDef> for TableSnapshot {
    fn from(table: &TableDef) -> Self {
        Self {
            name: table.name.to_owned(),
            columns: table.columns.iter().map(ColumnSnapshot::from).collect(),
        }
    }
}

impl From<&ColumnDef> for ColumnSnapshot {
    fn from(column: &ColumnDef) -> Self {
        Self {
            name: column.name.to_owned(),
            sql_type: column.sql_type.to_owned(),
            nullable: column.nullable,
        }
    }
}

fn path(root: &Path) -> PathBuf {
    root.join(".sqlwright").join("snapshot.json")
}
