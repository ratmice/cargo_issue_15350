Note that this bug is centered around recursive invocation of
cargo through the settings in `.cargo/config.toml`.

On a normal target: without runner specified in `config.toml`

Should `cargo test` fine.
```
cargo test
cargo run -p runner
```

I'm not sure how to write a simpler version of this test
that doesn't require wasmtime or wasm32-wasip2 target installed.


on wasm32-wasip2 where .cargo/config has the runner set to
invoke `cargo run -p runner` causing the following to fail.

```
cargo test --target wasm32-wasip2

```

Changing the `config.toml` to
```
[target.wasm32-wasip2]
runner = "target/debug/runner"
```

Both should all succeed.
```
cargo build
cargo test
cargo run -p runner
```


#### Ideally

Using `cargo run -p runner` would work that way we could avoid having to `cargo install`, to use a custom runner.
