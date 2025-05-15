use candid::{decode_args, decode_one, encode_args, encode_one, Principal};
use pocket_ic::PocketIc;

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
