use revm_delegate::RevmDelegate;

#[derive(RevmDelegate)]
#[revm_delegate(Database to &mut self.db where DB: revm::Database with { type Error = DB::Error; })]
#[revm_delegate(DatabaseRef to &self.db where DB: revm::DatabaseRef with { type Error = DB::Error; })]
#[revm_delegate(Inspector<DB> to &mut self.insp where DB: revm::Database, I: revm::Inspector<DB>)]
struct Delagator<DB, I> {
    db: DB,
    insp: I,
}
