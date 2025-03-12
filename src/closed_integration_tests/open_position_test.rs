use candid::Principal;

use super::*;

pub const _ONE_PERCENT: u64 = 100_000;

#[cfg(test)]
mod open_positon_fails {
    use super::*;

    #[test]
    fn test_that_position_can_not_be_when_canister_is_paused() {
        let admin = Principal::anonymous();

        let pic = _setup_market(admin);

        let trader0 = _get_principals()[0];

        let result = _open_position(&pic, trader0, 100000, false, OrderType::Limit, 10, None);

        assert!(matches!(result, Err(ref reason) if reason == "Market is paused"));
    }

    #[test]
    fn test_that_position_can_not_be_opened_when_collateral_is_less_than_min_collateral() {
        let admin = Principal::anonymous();

        let pic = _setup_market(admin);

        let min_collateral = 1000000;

        let _ = _set_state(&pic, admin, 199 * 100000, 20, min_collateral);

        let trader0 = _get_principals()[0];

        let result = _open_position(&pic, trader0, 100000, false, OrderType::Limit, 10, None);

        assert!(
            matches!(result, Err(ref reason) if reason == "Max leverage exceeded or collateral is too small")
        );
    }

    #[test]

    fn test_that_position_can_not_be_opened_if_user_already_has_a_position() {
        let admin = Principal::anonymous();

        let pic = _setup_market(admin);

        let min_collateral = 1_000_000;

        let _ = _set_state(&pic, admin, 199 * 100000, 100, min_collateral);

        let trader0 = _get_principals()[0];

        let limit_order_reference_tick = 200 * 100000;

        let _ = _open_position(
            &pic,
            trader0,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick),
        );

        let limit_order_reference_tick2 = 201 * 100000;

        let result = _open_position(
            &pic,
            trader0,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick2),
        );

        //println!("{:?}", result)

        assert!(
            matches!(result, Err(ref reason) if reason == "Account has pending error or unclosed position")
        );
    }
}

mod open_position_succeeds {
    use super::*;

    #[test]
    fn test_opening_limit_order_position_works_when_market_is_not_paused() {
        let admin = Principal::anonymous();

        let pic = _setup_market(admin);

        let min_collateral = 1_000_000;

        let _ = _set_state(&pic, admin, 199 * 100000, 100, min_collateral);

        let trader0 = _get_principals()[0];

        let limit_order_reference_tick = 200 * 100000;

        let _ = _open_position(
            &pic,
            trader0,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick),
        );

        let state_details = _get_state(&pic);
        //since its a short position ,the current tick which coinsides with the current price is shofted to be best sell offer

        assert_eq!(state_details.current_tick, limit_order_reference_tick);
    }

    #[test]
    fn test_current_tick_does_not_change_if_next_offer_is_not_better_than_previous_offer() {
        let admin = Principal::anonymous();

        let pic = _setup_market(admin);

        let min_collateral = 1_000_000;

        let _ = _set_state(&pic, admin, 199 * 100000, 100, min_collateral);

        let trader0 = _get_principals()[0];

        let limit_order_reference_tick1 = 200 * 100000;

        let _ = _open_position(
            &pic,
            trader0,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick1),
        );
        let state_details = _get_state(&pic);
        //since its a short position ,the current tick which coinsides with the current price is shifted to be best sell offer

        assert_eq!(state_details.current_tick, limit_order_reference_tick1);

        let trader1 = _get_principals()[1];

        let limit_order_reference_tick2 = 201 * 100000;

        let _ = _open_position(
            &pic,
            trader1,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick2),
        );

        let state_details = _get_state(&pic);

        // since the previous offer was a better offer ,the current tick (price) does not change
        assert_eq!(state_details.current_tick, limit_order_reference_tick1);
    }

    #[test]
    fn test_current_tick_changes_if_next_offer_is_better_than_previous_offer() {
        let admin = Principal::anonymous();

        let pic = _setup_market(admin);

        let min_collateral = 1_000_000;

        let _ = _set_state(&pic, admin, 199 * 100000, 100, min_collateral);

        let trader0 = _get_principals()[0];

        let limit_order_reference_tick1 = 200 * 100000;

        let _ = _open_position(
            &pic,
            trader0,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick1),
        );
        let state_details = _get_state(&pic);
        //since its a short position ,the current tick which coinsides with the current price is shifted to be best sell offer

        assert_eq!(state_details.current_tick, limit_order_reference_tick1);

        let trader1 = _get_principals()[1];

        let limit_order_reference_tick2 = 199 * 100000;

        let _ = _open_position(
            &pic,
            trader1,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick2),
        );

        let state_details = _get_state(&pic);

        assert_eq!(state_details.current_tick, limit_order_reference_tick2);
    }
}

mod test_open_market_position {

    use super::*;
    #[test]
    fn test_that_opening_long_market_position_shifts_current_price() {
        let admin = Principal::anonymous();

        let pic = _setup_market(admin);

        let min_collateral = 0;

        let _ = _set_state(&pic, admin, 199 * 100000, 100, min_collateral);

        let trader0 = _get_principals()[0];

        let limit_order_reference_tick0 = 200 * 100000;

        let _ = _open_position(
            &pic,
            trader0,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick0),
        );

        let state_details = _get_state(&pic);

        assert_eq!(state_details.current_tick, limit_order_reference_tick0);

        let trader1 = _get_principals()[1];

        let limit_order_reference_tick1 = 200 * 100000 + 5000;

        let _ = _open_position(
            &pic,
            trader1,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick1),
        );
        let current_state_details = _get_state(&pic);

        // price still remains the same
        assert_eq!(
            current_state_details.current_tick,
            limit_order_reference_tick0
        );

        let trader2 = _get_principals()[2];

        //max tick is in between limitorder reference tivck 0 and 1
        let max_tick = 200 * 100000 + 4000;

        let _ = _open_position(
            &pic,
            trader2,
            1_000_000_000_000,
            true,
            OrderType::Market,
            20,
            Some(max_tick),
        );

        let new_state_details = _get_state(&pic);

        assert_eq!(new_state_details.current_tick, limit_order_reference_tick1);
    }

    ///
    #[test]
    fn test_that_a_limit_order_position_can_be_filled_partially() {
        let admin = Principal::anonymous();

        let pic = _setup_market(admin);

        let min_collateral = 0;

        let _ = _set_state(&pic, admin, 199 * 100000, 100, min_collateral);

        let trader0 = _get_principals()[0];

        let limit_order_reference_tick1 = 200 * 100000;

        let _ = _open_position(
            &pic,
            trader0,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick1),
        );

        let trader1 = _get_principals()[1];

        let _ = _open_position(
            &pic,
            trader1,
            1_000_000_00,
            true,
            OrderType::Market,
            20,
            Some(limit_order_reference_tick1),
        );

        let (fully_filled, partially_filled) = _get_position_status(&pic, trader0);

        assert_eq!(fully_filled, false);
        assert!(partially_filled)
    }

    #[test]
    fn test_actove_position_pnl_is_() {
        let admin = Principal::anonymous();

        let pic = _setup_market(admin);

        let min_collateral = 0;

        let _ = _set_state(&pic, admin, 199 * 100000, 100, min_collateral);

        let trader0 = _get_principals()[0];

        let limit_order_reference_tick1 = 200 * 100000;

        let _ = _open_position(
            &pic,
            trader0,
            1_000_000_000,
            false,
            OrderType::Limit,
            20,
            Some(limit_order_reference_tick1),
        );

        let trader1 = _get_principals()[1];

        let _ = _open_position(
            &pic,
            trader1,
            1_000_000,
            true,
            OrderType::Market,
            30,
            Some(limit_order_reference_tick1),
        );

        let account = _get_user_account(&pic, trader1);

        let new_tick = 220 * 100000;

        _set_state(&pic, Principal::anonymous(), new_tick, 100, min_collateral);

        let pnl = _get_pnl(&pic, account);

        println!("The current pnl is {}", (pnl as f64) / (100000.0));
    }
}
