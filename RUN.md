# Running Locally

<p> Build and extract candid interface into .did file

```sh
cargo build --release --target wasm32-unknown-unknown --package market
candid-extractor target/wasm32-unknown-unknown/release/market.wasm > market.did
```

```sh
dfx generate market
```

```sh
dfx start
```