```sh
cargo build --release --target wasm32-unknown-unknown --package market

```

```sh
candid-extractor target/wasm32-unknown-unknown/release/market.wasm > market.did
```

type MarketDetails = record {
  vault_id : principal;
  collateral_decimal : nat8;
  quote_asset : Asset;
  base_asset : Asset;
  xrc_id : principal;
};

type Asset = record { class : AssetClass; symbol : text };

```sh

export VAULT_ID=$(dfx identity get-principal)
dfx deploy market --argument "(record {vault_id = principal \"${VAULT_ID}\";quote_asset = record {symbol = \"ICP\";class = variant {Cryptocurrency}};base_asset = record {symbol = \"BTC\";class = variant {Cryptocurrency}}; xrc_id = principal \"${VAULT_ID}\" })"
```

```sh
export CONTROLLER=$(dfx identity get-principal)
export SUBNET=csyj4-zmann-ys6ge-3kzi6-onexi-obayx-2fvak-zersm-euci4-6pslt-lae
dfx deploy token --argument "(variant {Init = record {burn_fee = 0 ;decimals = opt 8;token_symbol = \"ICP\";transfer_fee = 0;metadata = vec {};minting_account = record { owner = principal \"lmfrn-3iaaa-aaaaf-qaova-cai\" ; subaccount = null};
initial_balances = vec {};archive_options = record {num_blocks_to_archive = 1000;trigger_threshold = 2000;controller_id = principal \"${CONTROLLER}\";
cycles_for_archive_creation = opt 10000000000000;};token_name = \"ICP\";feature_flags = opt record{icrc2 =true};transfer_fee_rate = 0;burn_fee_rate = 0;fee_collector_account = null}
})"  --network ic --subnet ${SUBNET}
```
