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
invoke `cargo run -p runner`

```
cargo test --target wasm32-wasip2

```

removing the `cargo run` from the `config.toml`,
and invoking the runner directly should succeed.
