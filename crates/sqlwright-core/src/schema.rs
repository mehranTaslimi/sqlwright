#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnDef {
    pub name: &'static str,
    pub sql_type: &'static str,
    pub nullable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableDef {
    pub name: &'static str,
    pub columns: &'static [ColumnDef],
}
