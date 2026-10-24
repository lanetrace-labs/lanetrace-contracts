# LaneTrace slot-storage specimen

A small owner-controlled Soroban specimen exposing two intentionally different storage layouts. `bump_isolated(slot,amount)` updates only that slot. `bump_shared(slot,amount)` updates the slot and a shared aggregate. `read(slot)` and `global()` are read-only contract methods. Owner metadata remains in shared instance storage. Slots are bounded to 0–15, increments must be positive, and u64 additions use checked arithmetic.

The shared aggregate is meaningful extra state. The isolated layout does not maintain it; do not portray removing that write as a semantics-preserving optimization without application-specific analysis. This contract holds counters, not tokens or user funds.

Seven host tests exercise slot isolation, aggregate behavior, authorization, reads, overflow and input bounds. They test contract behavior, not validator parallel execution.

```sh
cargo test --locked
stellar contract build --locked
```

Soroban SDK 29.0.1 is pinned. The local build used Rust 1.97.1, wasm32v1-none and Stellar CLI 28.1.0. Build through Stellar CLI so required specification processing runs. Testnet deployment details and the precise evidence scope live in the sibling backend's TESTNET.md.

## Protocol reference

[CAP-0063](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0063.md) defines the footprint conflict rule and staged cluster semantics. LaneTrace implements the declared-key conflict rule and an illustrative clustering model only.
