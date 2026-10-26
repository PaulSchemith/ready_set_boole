/// a + b using only bitwise operators.
/// XOR is the sum without carry, AND shifted left is the carry; repeat until no carry is left.
pub fn adder(a: u32, b: u32) -> u32 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let carry = (a & b) << 1;
        a ^= b;
        b = carry;
    }
    a
}

/// a * b (wrapping on overflow) using shift-and-add built on `adder`.
pub fn multiplier(a: u32, b: u32) -> u32 {
    let (mut a, mut b) = (a, b);
    let mut result = 0;
    while b != 0 {
        if b & 1 != 0 {
            result = adder(result, a);
        }
        a <<= 1;
        b >>= 1;
    }
    result
}

pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_mul_gray() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(123, 877), 1000);
        assert_eq!(adder(u32::MAX, 1), 0);
        assert_eq!(multiplier(0, 5), 0);
        assert_eq!(multiplier(12345, 67), 12345 * 67);
        assert_eq!(multiplier(u32::MAX, 2), u32::MAX.wrapping_mul(2));
        let expected = [0, 1, 3, 2, 6, 7, 5, 4, 12];
        for (n, e) in expected.iter().enumerate() {
            assert_eq!(gray_code(n as u32), *e);
        }
    }
}
