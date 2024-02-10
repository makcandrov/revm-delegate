use revm_delegate::RevmDelegate;

#[derive(RevmDelegate)]
#[revm_delegate(Database to self.db where {DB: Database} with { type Error = DB::Error; })]
struct Delagator<DB> {
    db: DB,
}
