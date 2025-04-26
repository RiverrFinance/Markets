```sh
cargo build --release --target wasm32-unknown-unknown --package market

```

```sh
candid-extractor target/wasm32-unknown-unknown/release/market.wasm > market.did
```
