use candid::{decode_args, decode_one, encode_args, encode_one, Principal};
use pocket_ic::{PocketIc, WasmResult};

use std::cell::RefCell;

use std::fs;

use crate::{
    //  corelib::order_lib::LimitOrder,
    types::{Asset, AssetClass, MarketDetails, StateDetails, Tick},
    Amount, // OrderType, PositionDetails,
    OrderType,
    PositionParameters,
    Subaccount,
};

/// This is the testing framework for the market canister
/// Before running these tests ensure to turn off all inter cansiter calls as these tests should assume success on any intercanister call
///
///  

const _MARKET_WASM: &str = "target/wasm32-unknown-unknown/release/market.wasm";

thread_local! {
    static CANISTER_ID:RefCell<Principal> = RefCell::new(Principal::anonymous())
}

///////////////////////////////////////////////////////////////////////
/// Position Function
///////////////////////////////////////////////////////////////////////
fn _open_position(
    pic: &PocketIc,
    principal: Principal,
    _account_index: u8,
    collateral: Amount,
    long: bool,
    order_type: OrderType,
    leverage: u8,
    max_tick: Option<Tick>,
) -> Result<PositionParameters, String> {
    let canister_id = _get_canister_id();

    let returns;

    match pic.update_call(
        canister_id,
        principal,
        "openPosition",
        encode_args((
            _account_index,
            collateral,
            long,
            order_type,
            leverage,
            max_tick,
            1u64,
            1u64,
        ))
        .unwrap(),
    ) {
        Ok(reply) => {
            if let WasmResult::Reply(val) = reply {
                returns = val
            } else {
                return Err(String::from("error occured in canister "));
            }
        }
        Err(error) => {
            println!("{:?}", error);
            return Err(String::from("error opening position occured at pocket ic "));
        }
    }

    let reply = decode_one(&returns).unwrap();

    return reply;
}

fn _close_position(pic: &PocketIc, sender: Principal, _account_index: u8) -> u128 {
    let canister_id = _get_canister_id();

    let max_tick: Option<Tick> = Option::None;
    let Ok(WasmResult::Reply(res)) = pic.update_call(
        canister_id,
        sender,
        "closePosition",
        encode_args((_account_index, max_tick)).unwrap(),
    ) else {
        return 1234567890;
    };

    decode_one(&res).unwrap()
}

////////////////////////////////////////////////////////////////////////////////////
/// Getters
/////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////
fn _get_state(pic: &PocketIc) -> StateDetails {
    let canister_id = _get_canister_id();
    let Ok(WasmResult::Reply(val)) = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "getStateDetails",
        encode_one(()).unwrap(),
    ) else {
        panic!("error occured")
    };

    decode_one(&val).unwrap()
}

fn _get_position_status(pic: &PocketIc, subaccount: Subaccount) -> (bool, bool) {
    let canister_id = _get_canister_id();
    let Ok(WasmResult::Reply(val)) = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "positionStatus",
        encode_one(subaccount).unwrap(),
    ) else {
        panic!()
    };

    let reply = decode_args(&val).unwrap();

    return reply;
}

pub fn _get_best_offers(pic: &PocketIc) -> Result<(Tick, Tick), String> {
    let canister_id = _get_canister_id();

    let returns;

    match pic.query_call(
        canister_id,
        Principal::anonymous(),
        "getBestOffers",
        encode_one(()).unwrap(),
    ) {
        Ok(reply) => {
            if let WasmResult::Reply(val) = reply {
                returns = val
            } else {
                return Err(String::from("error occured in canister "));
            }
        }
        Err(error) => {
            println!("this error occured at pocket ic {:?}", error);
            return Err(String::from(
                "error getting best offers occured at pocket ic ",
            ));
        }
    }

    let reply = decode_args(&returns).unwrap();

    return Ok(reply);
}

// /// Get Position PNL
// fn _get_pnl(pic: &PocketIc, account: Subaccount) -> i64 {
//     let canister_id = _get_canister_id();
//     let Ok(WasmResult::Reply(val)) = pic.query_call(
//         canister_id,
//         Principal::anonymous(),
//         "getPositionPNL",
//         encode_one(account).unwrap(),
//     ) else {
//         panic!("Account could not be gotten")
//     };
//     let reply = decode_one(&val).unwrap();

//     return reply;
// }

/// Get Position Status
fn _get_user_account(pic: &PocketIc, principal: Principal, index: u8) -> Subaccount {
    let canister_id = _get_canister_id();
    let Ok(WasmResult::Reply(val)) = pic.query_call(
        canister_id,
        principal,
        "getUserAccount",
        encode_args((principal, index)).unwrap(),
    ) else {
        panic!("Account could not be gotten")
    };
    let reply = decode_one(&val).unwrap();

    return reply;
}

/// Get Account Position
fn _get_account_position(pic: &PocketIc, account: Subaccount) -> PositionParameters {
    let canister_id = _get_canister_id();
    let Ok(WasmResult::Reply(val)) = pic.query_call(
        canister_id,
        Principal::anonymous(),
        "getAccountPosition",
        encode_one(account).unwrap(),
    ) else {
        panic!("Account could not be gotten")
    };
    let reply = decode_one(&val).unwrap();

    return reply;
}

//////////////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////
///  Setters Function
/////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
fn _set_state(
    pic: &PocketIc,
    caller: Principal,
    current_tick: Tick,
    max_leveragex10: u8,
    min_collateral: Amount,
) {
    let canister_id = _get_canister_id();
    let state_details = StateDetails {
        not_paused: true,
        current_tick,
        max_leveragex10,
        min_collateral,
        base_token_multiple: 1,
    };

    let Ok(WasmResult::Reply(_)) = pic.update_call(
        canister_id,
        caller,
        "updateStateDetails",
        encode_one(state_details).unwrap(),
    ) else {
        panic!("error occured")
    };
}

fn _setup_market(admin: Principal) -> PocketIc {
    let pic = PocketIc::new();

    let perp_canister = pic.create_canister_with_settings(Some(admin), None);

    pic.add_cycles(perp_canister, 2_000_000_000_000); // 2T Cycles
                                                      //
    let wasm = fs::read(_MARKET_WASM).expect("Wasm file not found, run 'dfx build'.");

    let market_detais = MarketDetails {
        base_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: "ETH".to_string(),
        },
        quote_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: "ICP".to_string(),
        },
        xrc_id: admin,
        vault_id: admin,
    };

    pic.install_canister(
        perp_canister,
        wasm,
        encode_one(market_detais).unwrap(),
        Some(admin),
    );

    _set_canister_id(perp_canister);
    return pic;
}

fn _get_principals() -> Vec<Principal> {
    return vec![
        Principal::from_text("hpp6o-wqx72-gol5b-3bmzw-lyryb-62yoi-pjoll-mtsh7-swdzi-jkf2v-rqe")
            .unwrap(),
        Principal::from_text("cvwul-djb3r-e6krd-nbnfl-tuhox-n4omu-kejey-3lku7-ae3bx-icbu7-yae")
            .unwrap(),
        Principal::from_text("az6yt-a3f5b-k342j-5jncd-csa66-xfgvb-6f52c-jw5nh-o7g4k-bbf4q-vqe")
            .unwrap(),
        Principal::from_text("kmgqw-63fcp-ugexu-qgkyy-p3vjk-e4jh5-kyrdx-skajz-o3qra-wcyrb-qae")
            .unwrap(),
    ];
}

fn _get_canister_id() -> Principal {
    CANISTER_ID.with_borrow(|reference| reference.clone())
}

fn _set_canister_id(id: Principal) {
    CANISTER_ID.with_borrow_mut(|reference| *reference = id)
}

pub mod close_position_test;
pub mod open_position_test;
