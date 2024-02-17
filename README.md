# revm-delegate

Procedural macro to automatically delegate [`revm`](https://github.com/bluealloy/revm) traits.

## Features

- [`Database`](https://docs.rs/revm/latest/revm/trait.Database.html)
- [`DatabaseCommit`](https://docs.rs/revm/latest/revm/trait.DatabaseCommit.html)
- [`DatabaseRef`](https://docs.rs/revm/latest/revm/trait.DatabaseRef.html)
- [`Inspector`](https://docs.rs/revm/latest/revm/trait.Inspector.html)

## Usage

```rust
use revm_delegate::revm_delegate;
use revm::{Database, DatabaseCommit, DatabaseRef, Inspector};

#[revm_delegate(Database to &mut self.db where DB: Database with { type Error = DB::Error; })]
#[revm_delegate(DatabaseCommit to &mut self.db where DB: DatabaseCommit)]
#[revm_delegate(DatabaseRef to &self.db where DB: DatabaseRef with { type Error = DB::Error; })]
struct WrapDatabase<DB> {
    db: DB,
}

#[revm_delegate(Inspector<DB> to &mut self.insp where DB: Database, INSP: Inspector<DB>)]
struct WrapInspector<INSP> {
    insp: INSP,
}
```

## Note

Procedural macro auto-generated using [`delegate-trait`](https://github.com/makcandrov/delegate-trait).
