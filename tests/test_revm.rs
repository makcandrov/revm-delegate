use revm_delegate::RevmDelegate;

#[derive(RevmDelegate)]
#[revm_delegate(Database to &mut self.db where DB: revm::Database with { type Error = DB::Error; })]
#[revm_delegate(DatabaseRef to &self.db where DB: revm::DatabaseRef with { type Error = DB::Error; })]
#[revm_delegate(Inspector<DB2> to &mut self.insp where DB2: revm::Database, I: revm::Inspector<DB2>)]
struct Delagator<DB, I> {
    db: DB,
    insp: I,
}
