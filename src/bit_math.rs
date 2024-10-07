use crate::error::UniswapV3MathError;
use alloy::primitives::U256;

const M_128: U256 = U256::from_limbs([0, 0, 1, 0]);
const M_64: U256 = U256::from_limbs([0, 1, 0, 0]);
const M_32: U256 = U256::from_limbs([0x100000000, 0, 0, 0]);
const M_16: U256 = U256::from_limbs([0x10000, 0, 0, 0]);
const M_8: U256 = U256::from_limbs([0x100, 0, 0, 0]);
const M_4: U256 = U256::from_limbs([0x10, 0, 0, 0]);
const M_2: U256 = U256::from_limbs([0x4, 0, 0, 0]);
const M_1: U256 = U256::from_limbs([0x2, 0, 0, 0]);
const ZERO: U256 = U256::from_limbs([0, 0, 0, 0]);
const L_128: U256 = U256::from_limbs([0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0, 0]);
const L_64: U256 = U256::from_limbs([0xFFFFFFFFFFFFFFFF, 0, 0, 0]);
const L_32: U256 = U256::from_limbs([0xFFFFFFFF, 0, 0, 0]);
const L_16: U256 = U256::from_limbs([0xFFFF, 0, 0, 0]);
const L_8: U256 = U256::from_limbs([0xFF, 0, 0, 0]);
const L_4: U256 = U256::from_limbs([0xF, 0, 0, 0]);
const L_2: U256 = U256::from_limbs([0x3, 0, 0, 0]);
const L_1: U256 = U256::from_limbs([0x1, 0, 0, 0]);

pub fn most_significant_bit(x: U256) -> Result<u8, UniswapV3MathError> {
    if x.is_zero() {
        return Err(UniswapV3MathError::ZeroValue);
    }
    
    let mut x = x;
    let mut r = 0 as u8;
    
    if x >= M_128 {
        x >>= 128;
        r += 128;
    }

    if x >= M_64 {
        x >>= 64;
        r += 64;
    }

    if x >= M_32 {
        x >>= 32;
        r += 32;
    }

    if x >= M_16 {
        x >>= 16;
        r += 16;
    }

    if x >= M_8 {
        x >>= 8;
        r += 8;
    }

    if x >= M_4 {
        x >>= 4;
        r += 4;
    }

    if x >= M_2 {
        x >>= 2;
        r += 2;
    }

    if x >= M_1 {
        r += 1;
    }

    Ok(r)
}

pub fn least_significant_bit(x: U256) -> Result<u8, UniswapV3MathError> {
    if x.is_zero() {
        return Err(UniswapV3MathError::ZeroValue);
    }
    
    let mut x = x;
    let mut r = 255 as u8;

    if (x & L_128) > ZERO {
        r -= 128;
    } else {
        x >>= 128;
    }

    if (x & L_64) > ZERO {
        r -= 64;
    } else {
        x >>= 64;
    }

    if (x & L_32) > ZERO {
        r -= 32;
    } else {
        x >>= 32;
    }

    if (x & L_16) > ZERO {
        r -= 16;
    } else {
        x >>= 16;
    }

    if (x & L_8) > ZERO {
        r -= 8;
    } else {
        x >>= 8;
    }

    if (x & L_4) > ZERO {
        r -= 4;
    } else {
        x >>= 4;
    }

    if (x & L_2) > ZERO {
        r -= 2;
    } else {
        x >>= 2;
    }

    if (x & L_1) > ZERO {
        r -= 1;
    }

    Ok(r)
}

#[cfg(test)]
mod test {
    use super::most_significant_bit;
    use crate::{bit_math::least_significant_bit, U256_1};
    use alloy::primitives::U256;
    use std::str::FromStr;

    #[test]
    fn test_most_significant_bit() {
        //0
        let result = most_significant_bit(U256::ZERO);
        assert_eq!(
            result.unwrap_err().to_string(),
            "Can not get most significant bit or least significant bit on zero value"
        );

        //1
        let result = most_significant_bit(U256_1);
        assert_eq!(result.unwrap(), 0);

        //2
        let result = most_significant_bit(U256::from(2));
        assert_eq!(result.unwrap(), 1);

        //all powers of 2
        for i in 0..=255 {
            let result = most_significant_bit(U256::from(2).pow(U256::from(i)));
            assert_eq!(result.unwrap(), i as u8);
        }

        //uint256(-1)
        let result = most_significant_bit(
            //TODO:FIXME: might need to be from dec string
            U256::from_str(
                "115792089237316195423570985008687907853269984665640564039457584007913129639935",
            )
            .unwrap(),
        );
        assert_eq!(result.unwrap(), 255);
    }

    #[test]
    fn test_least_significant_bit() {
        //0
        let result = least_significant_bit(U256::ZERO);
        assert_eq!(
            result.unwrap_err().to_string(),
            "Can not get most significant bit or least significant bit on zero value"
        );

        //1
        let result = least_significant_bit(U256_1);
        assert_eq!(result.unwrap(), 0);

        //2
        let result = least_significant_bit(U256::from(2));
        assert_eq!(result.unwrap(), 1);

        //all powers of 2
        for i in 0..=255 {
            let result = least_significant_bit(U256::from(2).pow(U256::from(i)));
            assert_eq!(result.unwrap(), i as u8);
        }

        //uint256(-1)
        let result = least_significant_bit(
            //TODO:FIXME: might need to be from dec string
            U256::from_str(
                "115792089237316195423570985008687907853269984665640564039457584007913129639935",
            )
            .unwrap(),
        );
        assert_eq!(result.unwrap(), 0);
    }
}
