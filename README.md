A de/parsing and de/serialization library of [event data](https://gitlab.novisci.com/nsStat/event-data-model) written in [Rust](https://www.rust-lang.org/).

# building, testing, benchmarking

Build library:

```rust
cargo build
````

Run tests:

```rust
cargo test
```

Run benchmarks using [criterion](https://bheisler.github.io/criterion.rs/book/getting_started.html) crate. After running `cargo bench`, see `target/criterion/report/index.html` for results.


