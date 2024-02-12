use revm::{Database, DatabaseCommit, DatabaseRef, Inspector};
use revm_delegate::RevmDelegate;

#[derive(RevmDelegate)]
#[revm_delegate(Database to &mut self.db where DB: Database with { type Error = DB::Error; })]
#[revm_delegate(DatabaseCommit to &mut self.db where DB: DatabaseCommit)]
#[revm_delegate(DatabaseRef to &self.db where DB: DatabaseRef with { type Error = DB::Error; })]
#[revm_delegate(Inspector<DB2> to &mut self.insp where DB2: Database, I: Inspector<DB2>)]
struct WrapDatabaseAndInsepctor<DB, I> {
    db: DB,
    insp: I,
}
