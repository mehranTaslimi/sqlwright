use crate::snapshot::{ColumnSnapshot, TableSnapshot};

#[derive(Debug, PartialEq)]
pub enum Change<'a> {
    CreateSchema {
        name: &'a str,
    },
    DropSchema {
        name: &'a str,
    },
    RenameSchema {
        from: &'a str,
        to: &'a str,
    },

    CreateEnum {
        name: &'a str,
        values: &'a [String],
    },
    DropEnum {
        name: &'a str,
    },
    RenameEnum {
        from: &'a str,
        to: &'a str,
    },
    AddEnumValue {
        name: &'a str,
        value: &'a str,
        after: Option<&'a str>,
    },
    RenameEnumValue {
        name: &'a str,
        from: &'a str,
        to: &'a str,
    },

    CreateSequence {
        name: &'a str,
    },
    DropSequence {
        name: &'a str,
    },
    RenameSequence {
        from: &'a str,
        to: &'a str,
    },

    CreateTable(&'a TableSnapshot),
    DropTable {
        name: &'a str,
    },
    RenameTable {
        from: &'a str,
        to: &'a str,
    },
    SetTableComment {
        table: &'a str,
        comment: Option<&'a str>,
    },

    AddColumn {
        table: &'a str,
        column: &'a ColumnSnapshot,
    },
    DropColumn {
        table: &'a str,
        column: &'a str,
    },
    RenameColumn {
        table: &'a str,
        from: &'a str,
        to: &'a str,
    },
    AlterColumnType {
        table: &'a str,
        column: &'a str,
        from: &'a str,
        to: &'a str,
        using: Option<&'a str>,
    },
    SetNotNull {
        table: &'a str,
        column: &'a str,
    },
    DropNotNull {
        table: &'a str,
        column: &'a str,
    },
    SetDefault {
        table: &'a str,
        column: &'a str,
        default: &'a str,
    },
    DropDefault {
        table: &'a str,
        column: &'a str,
    },
    AddIdentity {
        table: &'a str,
        column: &'a str,
        always: bool,
    },
    DropIdentity {
        table: &'a str,
        column: &'a str,
    },
    DropGenerated {
        table: &'a str,
        column: &'a str,
    },
    SetColumnComment {
        table: &'a str,
        column: &'a str,
        comment: Option<&'a str>,
    },

    AddPrimaryKey {
        table: &'a str,
        name: &'a str,
        columns: &'a [String],
    },
    DropPrimaryKey {
        table: &'a str,
        name: &'a str,
    },
    AddUnique {
        table: &'a str,
        name: &'a str,
        columns: &'a [String],
    },
    DropUnique {
        table: &'a str,
        name: &'a str,
    },
    AddForeignKey {
        table: &'a str,
        name: &'a str,
        columns: &'a [String],
        ref_table: &'a str,
        ref_columns: &'a [String],
        on_delete: Option<&'a str>,
        on_update: Option<&'a str>,
    },
    DropForeignKey {
        table: &'a str,
        name: &'a str,
    },
    AddCheck {
        table: &'a str,
        name: &'a str,
        expression: &'a str,
    },
    DropCheck {
        table: &'a str,
        name: &'a str,
    },
    RenameConstraint {
        table: &'a str,
        from: &'a str,
        to: &'a str,
    },

    CreateIndex {
        table: &'a str,
        name: &'a str,
        columns: &'a [String],
        unique: bool,
    },
    DropIndex {
        name: &'a str,
    },
    RenameIndex {
        from: &'a str,
        to: &'a str,
    },

    CreateView {
        name: &'a str,
        query: &'a str,
    },
    DropView {
        name: &'a str,
    },
    RenameView {
        from: &'a str,
        to: &'a str,
    },
}
