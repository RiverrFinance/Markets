use ic_cdk::api::time;

use super::constants::*;

type Amount = u128;

/// Calculate Interest Function
///
/// This function calculates the interest on a leveraged position since when it was filled
/// The interest is calculated on an hourly basis
///
/// Note:Interest only counts if position is older than one hour  

#[derive(Debug)]
pub enum InterestCalcError {
    Overflow,
    InvalidTime,
}

pub fn _calc_interest(debt: Amount, interest_rate: u32, start_time: u64) -> Amount {
    let mut fee: Amount = 0;
    let one_hour: u64 = 3600 * ((10u64).pow(9));
    let mut current_hour = start_time;
    let current_time = time();

    if start_time > current_time {
        return 0;
    }

    loop {
        match current_hour.checked_add(one_hour) {
            Some(next_hour) if next_hour < current_time => {
                fee = fee.saturating_add(
                    ((interest_rate as u128).saturating_mul(debt))
                        .checked_div(u128::from(100 * _ONE_PERCENT))
                        .unwrap_or(0),
                );
                current_hour = next_hour;
            }
            _ => break,
        }
    }

    fee
}

/// Calculates Shares
///
/// This function calculates the amount of shares given the amount of asset being put in ,the current total shares and the current net liquidity

pub fn _calc_shares(
    amount_in: Amount,
    init_total_shares: Amount,
    init_liquidity: Amount,
) -> Amount {
    if init_total_shares == 0 {
        return amount_in;
    }
    // unsafe
    return (amount_in * init_total_shares) / init_liquidity;
}

/// Calculate Shares Value
///
/// This function calculates the value of a particular share given the current  amount of shares  and the  current net liquidity
pub fn _calc_shares_value(
    shares: Amount,
    init_total_shares: Amount,
    init_liquidity: Amount,
) -> Amount {
    // unsafe
    return (shares * init_liquidity) / init_total_shares;
}

/// Percentage Functions
///
/// These functions  calculates percentages  

pub fn _percentage<T>(x: u64, value: T) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Div<Output = T> + From<u64>,
{
    ((T::from(x)) * value) / T::from(100 * _ONE_PERCENT)
}
