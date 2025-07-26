use candid::Principal;
use pocket_ic::PocketIc;

use std::cell::RefCell;

use std::fs;

/// This is the testing framework for the market canister
/// Before running these tests ensure to turn off all inter cansiter calls as these tests should assume success on any intercanister call
///
///  

const _MARKET_WASM: &str = "target/wasm32-unknown-unknown/release/market.wasm";

thread_local! {
    static CANISTER_ID:RefCell<Principal> = RefCell::new(Principal::anonymous())
}

#[test]
fn setup() {
    let pic = PocketIc::new();
}
