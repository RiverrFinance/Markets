use super::constants::_PRICE_FACTOR;

type Amount = u128;

pub fn _equivalent(amount: Amount, price: u64, buy: bool) -> Amount {
    // unsafe
    if buy {
        let result = (amount * _PRICE_FACTOR) / price as u128;
        if result == _equivalent(result, price, false) {
            return result;
        } else {
            return (((amount as f64) / price as f64) * _PRICE_FACTOR as f64) as u128;
        }
    } else {
        return (amount * price as u128) / _PRICE_FACTOR;
    }
}
