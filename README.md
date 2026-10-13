# stable_digest

A stable, `no_std` content digest: SHA-256 of some bytes, rendered as lowercase hex.

Deliberately tiny and generic. Content-addressing has no business living inside a
diagnostics crate, so this is the shared primitive that [`liaise`](https://crates.io/crates/liaise)
and anything else needing a stable digest builds on.

```rust
use stable_digest::StableDigest;
let d = StableDigest::of(b"hello");
assert_eq!(d.as_str().len(), 64);
```

## License

MIT or Apache-2.0, at your option.
