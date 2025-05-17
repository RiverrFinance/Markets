use super::bit_lib::{_least_sigbit_position, _most_sigbit_position};
use super::tick_lib::{_next_default_tick, _tick_zero};

use super::constants::{_ONE_BASIS_POINT, _ONE_PERCENT};

/// Flip Bit
///
/// This function is used to flip a particlar bit on a bitmap,
/// it either initialises it if it's not initialised or the reverse

pub fn _flip_bit(bitmap: u128, bit_position: u64) -> u128 {
    if bit_position == 0 {
        return bitmap;
    }
    let mask = 1 << (99 - bit_position);
    return bitmap ^ mask;
}

/// Next Initialised Tick
///
/// This function is used to calculate the next initialised tick from  the bitmap of an integral
///
/// Note
///  - This function returns the next default tick (see tick_lib) if no tick is initialised within the bitmap

pub fn _next_initialised_tick(bitmap: u128, bit_position: u64, integral: u64, buy: bool) -> u64 {
    let reference = 99 - bit_position;
    if buy {
        let mask = ((1u128) << reference) - 1;
        let masked = bitmap & mask;

        if masked == 0 {
            return _next_default_tick(integral, true); // (integral + 1) * _ONE_PERCENT * tick_spacing;
        } else {
            return (integral * _ONE_PERCENT) + (_most_sigbit_position(masked) * _ONE_BASIS_POINT);
        }
    } else {
        let mask = !(((1u128) << (reference + 1)) - 1);
        let masked = mask & bitmap;

        if masked == 0 {
            if bit_position == 0 {
                return _next_default_tick(integral, false);
            }

            return _tick_zero(integral); // (integral - 1) * _ONE_PERCENT + (99 * _ONE_BASIS_POINT)
        } else {
            return (integral * _ONE_PERCENT) + (_least_sigbit_position(masked) * _ONE_BASIS_POINT);
        }
    }
}
