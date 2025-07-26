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
export INIT_CYCLES=4000000000000
# export SUBNET=csyj4-zmann-ys6ge-3kzi6-onexi-obayx-2fvak-zersm-euci4-6pslt-lae
dfx canister create market --with-cycles ${INIT_CYCLES}  # --network ic --subnet ${SUBNET} 
```

```sh
export VAULT_ID=5se5w-zaaaa-aaaaf-qanmq-cai
export XRC_ID=uf6dk-hyaaa-aaaaq-qaaaq-cai
dfx deploy market --argument "(record {vault_id = principal \"${VAULT_ID}\";quote_asset = record {symbol = \"ICP\";class = variant {Cryptocurrency}};base_asset = record {symbol = \"BTC\";class = variant {Cryptocurrency}}; xrc_id = principal \"${XRC_ID}\";tick_spacing = 100 })"   --network ic 
```

```sh
dfx canister call market updateStateDetails "(record {max_leveragex10 = 100; not_paused = true; min_collateral = 0})"  --network ic 
```

```sh
dfx canister uninstall-code market   --network ic
```