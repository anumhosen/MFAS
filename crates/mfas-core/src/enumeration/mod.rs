//! Mathematical Enumeration Module: Bijective mapping between $\mathbb{N}_0$ and $B^*$
//!
//! Provides a canonical, length-sensitive bijection between natural numbers
//! and finite byte strings.
//!
//! Invariant:
//! 1. `number_to_bytes(bytes_to_number(b)) == b` for all `b` in $B^*$
//! 2. `bytes_to_number(number_to_bytes(n)) == n` for all `n` in $\mathbb{N}_0$
//! 3. Different length sequences with identical trailing values (e.g. `[0x01]`, `[0x00, 0x01]`, `[0x00, 0x00, 0x01]`)
//!    map to distinct natural numbers.

use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Calculate the offset for strings of length L in bijective base-256:
/// $\text{Offset}(L) = \sum_{i=0}^{L-1} 256^i = \frac{256^L - 1}{255}$
pub fn length_offset(len: usize) -> BigUint {
    if len == 0 {
        return BigUint::zero();
    }
    let pow_256_l = BigUint::one() << (8 * len);
    (pow_256_l - BigUint::one()) / 255u32
}

/// Map a finite byte sequence to a unique natural number $N \in \mathbb{N}_0$.
pub fn bytes_to_number(bytes: &[u8]) -> BigUint {
    let len = bytes.len();
    if len == 0 {
        return BigUint::zero();
    }
    let offset = length_offset(len);
    let val = BigUint::from_bytes_be(bytes);
    offset + val
}

/// Map a natural number $N \in \mathbb{N}_0$ to its unique finite byte sequence in $B^*$.
pub fn number_to_bytes(n: &BigUint) -> Vec<u8> {
    if n.is_zero() {
        return Vec::new();
    }

    // M = 255 * N + 1
    let m = (n * 255u32) + BigUint::one();
    let bits = m.bits();
    let len = ((bits - 1) / 8) as usize;

    let offset = length_offset(len);
    let val = n - offset;

    let raw_bytes = val.to_bytes_be();
    if raw_bytes.len() < len {
        let mut padded = vec![0u8; len - raw_bytes.len()];
        padded.extend_from_slice(&raw_bytes);
        padded
    } else {
        raw_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string_maps_to_zero() {
        let empty = b"";
        let n = bytes_to_number(empty);
        assert_eq!(n, BigUint::zero());
        assert_eq!(number_to_bytes(&n), empty);
    }

    #[test]
    fn test_single_byte_range() {
        // [0x00] -> 1
        assert_eq!(bytes_to_number(&[0x00]), BigUint::from(1u32));
        assert_eq!(number_to_bytes(&BigUint::from(1u32)), vec![0x00]);

        // [0x01] -> 2
        assert_eq!(bytes_to_number(&[0x01]), BigUint::from(2u32));
        assert_eq!(number_to_bytes(&BigUint::from(2u32)), vec![0x01]);

        // [0xFF] -> 256
        assert_eq!(bytes_to_number(&[0xFF]), BigUint::from(256u32));
        assert_eq!(number_to_bytes(&BigUint::from(256u32)), vec![0xFF]);
    }

    #[test]
    fn test_two_byte_range() {
        // [0x00, 0x00] -> 257
        assert_eq!(bytes_to_number(&[0x00, 0x00]), BigUint::from(257u32));
        assert_eq!(number_to_bytes(&BigUint::from(257u32)), vec![0x00, 0x00]);

        // [0x00, 0x01] -> 258
        assert_eq!(bytes_to_number(&[0x00, 0x01]), BigUint::from(258u32));
        assert_eq!(number_to_bytes(&BigUint::from(258u32)), vec![0x00, 0x01]);

        // [0xFF, 0xFF] -> 65792
        assert_eq!(bytes_to_number(&[0xFF, 0xFF]), BigUint::from(65792u32));
        assert_eq!(number_to_bytes(&BigUint::from(65792u32)), vec![0xFF, 0xFF]);
    }

    #[test]
    fn test_distinguishes_leading_zeros() {
        let b1 = vec![0x01];
        let b2 = vec![0x00, 0x01];
        let b3 = vec![0x00, 0x00, 0x01];

        let n1 = bytes_to_number(&b1);
        let n2 = bytes_to_number(&b2);
        let n3 = bytes_to_number(&b3);

        assert_ne!(n1, n2);
        assert_ne!(n2, n3);
        assert_ne!(n1, n3);

        assert_eq!(number_to_bytes(&n1), b1);
        assert_eq!(number_to_bytes(&n2), b2);
        assert_eq!(number_to_bytes(&n3), b3);
    }

    #[test]
    fn test_arbitrary_data_roundtrip() {
        let test_cases: Vec<&[u8]> = vec![
            b"Hello",
            b"The quick brown fox jumps over the lazy dog",
            &[0, 0, 0, 0, 0, 0, 1],
            &[255, 128, 64, 32, 16, 8, 4, 2, 1, 0],
        ];

        for case in test_cases {
            let n = bytes_to_number(case);
            let restored = number_to_bytes(&n);
            assert_eq!(restored.as_slice(), case);
        }
    }

    #[test]
    fn test_sequential_natural_numbers() {
        for i in 0..1000u32 {
            let n = BigUint::from(i);
            let bytes = number_to_bytes(&n);
            let restored_n = bytes_to_number(&bytes);
            assert_eq!(restored_n, n, "Failed for integer {}", i);
        }
    }
}
