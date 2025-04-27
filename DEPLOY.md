```sh
export INIT_CYCLES=4000000000000
export SUBNET=csyj4-zmann-ys6ge-3kzi6-onexi-obayx-2fvak-zersm-euci4-6pslt-lae
dfx canister create market --network ic --subnet ${SUBNET} --with-cycles ${INIT_CYCLES}
```

```sh
export VAULT_ID=5se5w-zaaaa-aaaaf-qanmq-cai
export XRC_ID=uf6dk-hyaaa-aaaaq-qaaaq-cai
dfx deploy market --argument "(record {vault_id = principal \"${VAULT_ID}\";quote_asset = record {symbol = \"ICP\";class = variant {Cryptocurrency}};base_asset = record {symbol = \"BTC\";class = variant {Cryptocurrency}}; xrc_id = principal \"${XRC_ID}\" })"  --network ic --subnet ${SUBNET}
```

```sh
dfx canister call --network ic market updateStateDetails "(record {max_leveragex10 = 100; not_paused = true; current_tick = 0; base_token_multiple = 1; min_collateral = 0})"
```