/// Constants for each market ,adjustment should be made for these and these adjustments are not changed
///
/// ```
///
/// _PRICE_FACTOR :
/// /// The price Factor determines the tick representation ,default to 100% i.e 100 * _ONE_PERCENT
/// /// NOTE:
///  tick = price * _PRICE_FACTOR
/// _ONE_BASIS_POINT:
/// /// This is 0.01% ,it is also the unit of tick difference
/// _ONE_PERCENT:
/// /// This is 1% or 100 BASIS POINT
/// ```
///`

pub const _PRICE_FACTOR: u128 = 10_000_000;

pub const _ONE_BASIS_POINT: u64 = 1000;

pub const _ONE_PERCENT: u64 = 100_000;
//default 1
