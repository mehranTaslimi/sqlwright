use crate::{
    change::Change,
    snapshot::TableSnapshot,
    types::{Nullable, SqlType},
};

pub trait Backend: Sized + Send + Sync + 'static {
    type Database: sqlx::Database;

    fn render(change: &Change<'_>) -> String {
        use Change::*;

        match change {
            CreateTable(t) => Self::create_table(t),
            DropTable { name } => Self::drop_table(name),
            AddColumn { .. }
            | DropColumn { .. }
            | SetNotNull { .. }
            | DropNotNull { .. }
            | AlterColumnType { .. }
            | AddPrimaryKey { .. }
            | DropPrimaryKey { .. }
            | AddIdentity { .. }
            | DropIdentity { .. }
            | SetDefault { .. }
            | DropDefault { .. }
            | AddUnique { .. }
            | DropUnique { .. }
            | AddForeignKey { .. }
            | DropForeignKey { .. }
            | CreateIndex { .. }
            | DropIndex { .. }
            | CreateEnum { .. }
            | DropEnum { .. }
            | AddEnumValue { .. }
            | AddCheck { .. }
            | DropCheck { .. }
            | CreateSchema { .. }
            | DropSchema { .. }
            | CreateSequence { .. }
            | DropSequence { .. }
            | CreateView { .. }
            | DropView { .. }
            | DropGenerated { .. }
            | SetTableComment { .. }
            | SetColumnComment { .. }
            | RenameTable { .. }
            | RenameColumn { .. }
            | RenameConstraint { .. }
            | RenameIndex { .. }
            | RenameEnum { .. }
            | RenameEnumValue { .. }
            | RenameSchema { .. }
            | RenameSequence { .. }
            | RenameView { .. } => unimplemented!("render {change:?}"),
        }
    }

    fn quote_ident(name: &str) -> String;
    fn create_table(table: &TableSnapshot) -> String;
    fn drop_table(name: &str) -> String;
}

#[diagnostic::on_unimplemented(
    message = "the `{Self}` backend does not support the SQL type `{S}`",
    label = "`{S}` is not supported by `{Self}`",
    note = "a backend crate adds support with `impl HasSqlType<{S}> for {Self} {{}}`"
)]
pub trait HasSqlType<S: SqlType>: Backend {
    const TYPE_NAME: &'static str;
    const NULLABLE: bool = false;
}

impl<B, S> HasSqlType<Nullable<S>> for B
where
    B: HasSqlType<S>,
    S: SqlType,
{
    const TYPE_NAME: &'static str = <B as HasSqlType<S>>::TYPE_NAME;
    const NULLABLE: bool = true;
}
