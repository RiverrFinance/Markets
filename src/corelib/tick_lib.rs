use super::calc_lib::_percentage;
use super::constants::*;

/// Default Max Tick
///
/// Gets the default max tick for a particular trade direction (buy or sell)
///
/// This is currently implemented as a 0.5 percent incerase or decrease from the starting_tick

pub fn _def_max_tick(current_tick: u64, buy: bool) -> u64 {
    let delta = _percentage(50 * _ONE_BASIS_POINT, current_tick);
    if buy {
        current_tick + delta
    } else {
        current_tick - delta
    }
}

/// Next Default Tick       
///
///
pub fn _next_default_tick(integral: u64, buy: bool) -> u64 {
    if buy {
        _tick_zero(integral + 1)
    } else {
        _tick_zero(integral - 1) + (99 * _ONE_BASIS_POINT)
    }
}

/// Tick Zero
///
/// The tick zero of an integral corresponds to the tick with that integral  and a  of 0 i.e whole percentages (1%,3% etc)
pub fn _tick_zero(integral: u64) -> u64 {
    integral * (_ONE_PERCENT)
}

/// Mul and Bit
///
/// This function is used to calculate the integral and decimal pert of a tick

pub fn _int_and_dec(tick: u64) -> (u64, u64) {
    let multiplier = tick / _ONE_PERCENT;
    let bit_position = (tick % _ONE_PERCENT) / (_ONE_BASIS_POINT);
    return (multiplier, bit_position);
}

pub fn _compressed_tick(tick: u64, tick_spacing: u64) -> u64 {
    return tick / tick_spacing;
}

/// Excceded Stopping Tick
/// This functions checks that stoping tick is not exceeded in the particular swap direction
///
///  

pub fn _exceeded_stopping_tick(current_tick: u64, stopping_tick: u64, buy: bool) -> bool {
    if buy {
        return current_tick > stopping_tick;
    } else {
        return current_tick < stopping_tick;
    }
}
