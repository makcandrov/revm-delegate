# revm-delegate

Procedural macro to automatically delegate [`revm`](https://github.com/bluealloy/revm) traits.

## Usage

```rust
use revm_delegate::RevmDelegate;
use revm::{Database, DatabaseRef};

#[derive(RevmDelegate)]
#[revm_delegate(Database to &mut self.db where DB: Database with { type Error = DB::Error; })]
#[revm_delegate(DatabaseRef to &self.db where DB: DatabaseRef with { type Error = DB::Error; })]
struct WrapDatabase<DB> {
    db: DB,
}
```

## Notice

Procedural macro auto-generated using [`delegate-trait`](https://github.com/makcandrov/delegate-trait).
