use super::bitmap_lib::{_flip_bit, _next_initialised_tick};
use super::price_lib::_equivalent;
use super::tick_lib::*;
use crate::types::{TickDetails, TickState};

use ic_stable_structures::{memory_manager::VirtualMemory, DefaultMemoryImpl, StableBTreeMap};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type Tick = u64;
type Amount = u128;
type MB = StableBTreeMap<u64, u128, Memory>;
type TD = StableBTreeMap<u64, TickDetails, Memory>;

/// Get Best Offer
///
///Gets the best offer for either selling or buying

pub fn _get_best_offer<'a>(
    buy: bool,
    current_tick: Tick,
    stopping_tick: Tick,
    tick_spacing: u64,
    integrals_bitmaps: &'a mut MB,
    ticks_details: &'a mut TD,
) -> Option<Tick> {
    let mut resulting_tick = 0;
    let mut loop_current_tick = current_tick;
    while !(_exceeded_stopping_tick(loop_current_tick, stopping_tick, buy)) {
        let (integral, bit_position) = _int_and_dec(loop_current_tick, tick_spacing);
        let bitmap = match integrals_bitmaps.get(&integral) {
            Some(res) => res,
            None => {
                // if integral has no bitmap means that means  no tick within that integral   is
                //initialised

                // calculates the  next_default tick (See bitmap_lib)
                // if next default tick exceeds stopping tick
                //breaks else
                // updates current tick to the next default tick

                let next_default_tick = _next_default_tick(integral, tick_spacing, buy);

                loop_current_tick = next_default_tick;
                //stops currrent iteration,starts the next at the next default tick
                continue;
            }
        };
        let tick_details = match ticks_details.get(&loop_current_tick) {
            Some(res) => res,
            None => {
                let next_initialised_tick =
                    _next_initialised_tick(bitmap, bit_position, integral, tick_spacing, buy);

                loop_current_tick = next_initialised_tick;
                continue;
            }
        };

        // this checks that the swap is in the right direction
        // i.e if buying then the tick state must be sell and vice versa
        let right_direction = match tick_details.tick_state {
            TickState::SELL => buy == true,
            TickState::BUY => buy == false,
        };

        let liquidity_boundary = tick_details.liq_bounds;

        if liquidity_boundary._liquidity_within() > 0 && right_direction {
            resulting_tick = loop_current_tick;
            break;
        }

        let next_initialised_tick =
            _next_initialised_tick(bitmap, integral, bit_position, tick_spacing, buy);

        loop_current_tick = next_initialised_tick;
    }

    if resulting_tick == 0 {
        return None;
    } else {
        return Some(resulting_tick);
    }
}

struct SwapTickConstants {
    tick: Tick,
    order_size: Amount,
}

/// SwapParams for initiating  a swap
/// utilsed for opening position at market price
pub struct SwapParams<'a> {
    /// Swap Direction
    ///
    /// true if buying or false if selling
    pub buy: bool,
    /// Init Tick
    ///
    /// the current state tick of the market ,also seen as current price
    ///
    /// this also translates to the current price
    pub init_tick: Tick,
    ///Stopping Tick
    ///
    /// stopping tick at which swapping should not not exceed
    ///
    /// This can be viewed as maximum excecution price for a market order
    /// if specified swap does not exceed this and returns the net previous amount from ticks below and the amount remaining
    pub stopping_tick: Tick,
    /// Tick Spacing
    ///
    /// magnitude of difference in basis point between two neigbouring ticks
    pub tick_spacing: u64,
    /// Order Size
    ///
    /// the amount of asset being swapped
    pub order_size: Amount,
    /// Multiplier BitMaps
    ///
    /// HashMap  of integrals to their bitmaps
    pub integrals_bitmaps: &'a mut MB,
    /// Ticks Details
    ///
    /// HashMasp  of ticks to their  respective tick_details
    pub ticks_details: &'a mut TD,
}

impl<'a> SwapParams<'a> {
    /// Swap Function
    ///
    /// Swap is executed as a loop starting at the current tick till stopping tick is reached is exceeded
    ///
    /// Returns
    ///  - AmountOut :The amount of token gotten from the swap
    ///  - AmountRemaining :The amount of asset remaining dues to swap not being completely filled before stopping tick
    ///  - Current or Resulting Tick : This corresponds to the tick at which either asset was fully swapped
    /// or tick before stopping tick was exceeded
    ///
    ///  - Crossed Ticks :The Total ticks that were crossed
    pub fn _swap(&mut self) -> (Amount, Amount, Tick, Vec<Tick>) {
        let mut amount_out = 0;

        let mut amount_remaining = self.order_size;

        let mut resulting_tick = self.init_tick;

        let mut crossed_ticks: Vec<Tick> = Vec::new();

        let mut loop_current_tick = self.init_tick;

        while !(_exceeded_stopping_tick(loop_current_tick, self.stopping_tick, self.buy)) {
            let (integral, bit_position) = _int_and_dec(loop_current_tick, self.tick_spacing);

            let bitmap = match self.integrals_bitmaps.get(&integral) {
                Some(res) => res,
                None => {
                    // if integral has no bitmap means that means  no tick within that integral   is
                    //initialised

                    // calculates the  next_default tick (See bitmap_lib)
                    // if next default tick exceeds stopping tick
                    //breaks else
                    // updates current tick to the next default tick

                    let next_default_tick =
                        _next_default_tick(integral, self.tick_spacing, self.buy);
                    if _exceeded_stopping_tick(next_default_tick, self.stopping_tick, self.buy) {
                        break;
                    };

                    loop_current_tick = next_default_tick;
                    //stops currrent iteration,starts the next at the next default tick
                    continue;
                }
            };

            let tick_params = SwapTickConstants {
                order_size: amount_remaining,
                tick: loop_current_tick,
            };

            let (value_out, boundary_closed);

            if self.buy {
                (value_out, amount_remaining, boundary_closed) = self._buy_at_tick(tick_params);
            } else {
                (value_out, amount_remaining, boundary_closed) = self._sell_at_tick(tick_params);
            }

            if value_out > 0 {
                amount_out += value_out;

                resulting_tick = loop_current_tick;

                // if static liquidity was exhausted at that tick

                if boundary_closed {
                    self.ticks_details.remove(&loop_current_tick);
                    //add ticks to list of crossed ticks
                    crossed_ticks.push(loop_current_tick);

                    let flipped_bitmap = _flip_bit(bitmap, bit_position);

                    let tick_zero = _tick_zero(integral, self.tick_spacing);
                    // if flipping bitmap results in zero and tick zero(see bitmap_lib) is not contained in ticks_details hashmap
                    //delete btimap
                    if flipped_bitmap == 0 && !self.ticks_details.contains_key(&tick_zero) {
                        self.integrals_bitmaps.remove(&integral);
                    } else {
                        // insert flipped bitmap
                        self.integrals_bitmaps.insert(integral, flipped_bitmap);
                    };
                }

                if amount_remaining == 0 {
                    break;
                }
            }

            //println!()
            let next_initialised_tick =
                _next_initialised_tick(bitmap, integral, bit_position, self.tick_spacing, self.buy);

            loop_current_tick = next_initialised_tick;
        }
        // if swap could not happen ,current tick remains unchanged and can only be changed manually

        return (amount_out, amount_remaining, resulting_tick, crossed_ticks);
    }

    /// buy at tick function
    ///
    /// Performs a swap at a particular tick
    ///
    /// Returns
    /// - AmountOut :The amount resulting from the swap for a buy order at that tick
    /// - AmountRemaining :The amount remaining from swapping at that tick ,
    ///  this is  zero if the swap is completedly fully at tick
    /// - Cleared : true if all liquidity at tick  was cleared
    /// - Boundary Closed : true if all static liquidity at tick (see TickDetails and LiquidityBoundary) is cleared
    fn _buy_at_tick(&mut self, params: SwapTickConstants) -> (Amount, Amount, bool) {
        let mut amount_out = 0;

        let mut amount_remaining = params.order_size;

        let boundary_closed;

        // let tick_price = _tick_to_price(params.tick);

        let equivalent =
            |amount: Amount, buy: bool| -> Amount { _equivalent(amount, params.tick, buy) };

        let mut tick_details = match self.ticks_details.get(&params.tick) {
            Some(res) => res,
            None => return (amount_out, amount_remaining, false),
        };

        if let TickState::BUY = tick_details.tick_state {
            return (amount_out, amount_remaining, false);
        }

        let init_tick_liq = tick_details.liq_bounds._liquidity_within();

        //value of all_liquidity in token1
        let init_liq_equivalent = equivalent(init_tick_liq, false);

        if init_liq_equivalent <= self.order_size {
            // all liquidity has been exhausted
            amount_out = init_tick_liq;

            amount_remaining -= init_liq_equivalent;
        } else {
            //liquidity remains
            amount_out = equivalent(self.order_size, true);

            amount_remaining = 0;
        }

        tick_details.liq_bounds._reduce_boundary(amount_out);

        boundary_closed = tick_details.liq_bounds._liquidity_within() == 0;

        self.ticks_details.insert(params.tick, tick_details);

        return (amount_out, amount_remaining, boundary_closed);
    }

    /// Sell at tick function
    ///
    /// Performs a swap at a particular tick
    ///
    /// Returns
    /// - AmountOut :The amount resulting from the swap for a sell order at that tick
    /// - AmountRemaining :The amount remaining from swapping at that tick ,
    ///  this is  zero if the swap is completedly fully at tick
    /// - Cleared : true if all liquidity at tick  was cleared
    /// - Boundary Closed : true if all static liquidity at tick (see TickDetails and LiquidityBoundary) is cleared

    fn _sell_at_tick(&mut self, params: SwapTickConstants) -> (Amount, Amount, bool) {
        let mut amount_out = 0;

        let mut amount_remaining = params.order_size;

        let boundary_closed;

        //  let tick_price = _tick_to_price(params.tick);

        let equivalent =
            |amount: Amount, buy: bool| -> Amount { _equivalent(amount, params.tick, buy) };

        // tick details
        let mut tick_details = match self.ticks_details.get(&params.tick) {
            Some(res) => res,
            None => return (amount_out, amount_remaining, false),
        };

        if let TickState::SELL = tick_details.tick_state {
            return (amount_out, amount_remaining, false);
        }

        let init_tick_liq = tick_details.liq_bounds._liquidity_within();

        let init_liq_equivalent = equivalent(init_tick_liq, true);

        if init_liq_equivalent <= self.order_size {
            amount_out = init_tick_liq;

            amount_remaining -= init_liq_equivalent;
        } else {
            //liquidity remains
            amount_out = equivalent(self.order_size, false);

            amount_remaining = 0;
        }

        tick_details.liq_bounds._reduce_boundary(amount_out);

        boundary_closed = tick_details.liq_bounds._liquidity_within() == 0;

        self.ticks_details.insert(params.tick, tick_details);

        return (amount_out, amount_remaining, boundary_closed);
    }
}

// #[test]
// fn testing() {
//     let tick = 10024_000;

//     let price = _tick_to_price(tick);

//     println!("{}", price);

//     let amount = 103020000;

//     let equiavlent = _equivalent(amount, price, false);

//     println!("{}", equiavlent);

//     let amount2 = _equivalent(equiavlent, price, true);

//     println!("{}", amount2)

//     // assert_eq!(amount, amount2)
// }
