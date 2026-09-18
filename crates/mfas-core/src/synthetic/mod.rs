//! Synthetic Data Generator
//!
//! Provides deterministic streaming readers for mathematical and stress testing data:
//! - Zeros: constant zero streams
//! - Counter: sequential byte streams ($0, 1, 2, \dots, 255, 0, \dots$)
//! - Repeat: repeated custom byte patterns
//! - Random: deterministic pseudo-random streams using Xorshift64

use std::io::Read;

/// Deterministic synthetic data generator stream
pub struct SyntheticStream {
    generator: GeneratorKind,
    remaining: u64,
}

enum GeneratorKind {
    Zeros,
    Counter { current: u8 },
    Repeat { pattern: Vec<u8>, offset: usize },
    Random { state: u64 },
}

impl SyntheticStream {
    /// Stream of all zero bytes
    pub fn zeros(total_bytes: u64) -> Self {
        Self {
            generator: GeneratorKind::Zeros,
            remaining: total_bytes,
        }
    }

    /// Stream of incrementing byte values
    pub fn counter(total_bytes: u64) -> Self {
        Self {
            generator: GeneratorKind::Counter { current: 0 },
            remaining: total_bytes,
        }
    }

    /// Stream repeating a specific byte pattern
    pub fn repeat(pattern: Vec<u8>, count: u64) -> Self {
        let total = (pattern.len() as u64).saturating_mul(count);
        Self {
            generator: GeneratorKind::Repeat { pattern, offset: 0 },
            remaining: total,
        }
    }

    /// Deterministic pseudo-random stream using XorShift64
    pub fn random(total_bytes: u64, seed: u64) -> Self {
        let state = if seed == 0 { 0xDEADBEEFCAFEBABE } else { seed };
        Self {
            generator: GeneratorKind::Random { state },
            remaining: total_bytes,
        }
    }

    pub fn total_remaining(&self) -> u64 {
        self.remaining
    }
}

impl Read for SyntheticStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.remaining == 0 || buf.is_empty() {
            return Ok(0);
        }

        let to_write = (buf.len() as u64).min(self.remaining) as usize;
        let slice = &mut buf[..to_write];

        match &mut self.generator {
            GeneratorKind::Zeros => {
                slice.fill(0);
            }
            GeneratorKind::Counter { current } => {
                for byte in slice.iter_mut() {
                    *byte = *current;
                    *current = current.wrapping_add(1);
                }
            }
            GeneratorKind::Repeat { pattern, offset } => {
                if pattern.is_empty() {
                    return Ok(0);
                }
                for byte in slice.iter_mut() {
                    *byte = pattern[*offset];
                    *offset = (*offset + 1) % pattern.len();
                }
            }
            GeneratorKind::Random { state } => {
                for byte in slice.iter_mut() {
                    // Xorshift64 algorithm
                    let mut x = *state;
                    x ^= x << 13;
                    x ^= x >> 7;
                    x ^= x << 17;
                    *state = x;
                    *byte = (x & 0xFF) as u8;
                }
            }
        }

        self.remaining -= to_write as u64;
        Ok(to_write)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros_stream() {
        let mut stream = SyntheticStream::zeros(100);
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        assert_eq!(buf.len(), 100);
        assert!(buf.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_counter_stream() {
        let mut stream = SyntheticStream::counter(300);
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        assert_eq!(buf.len(), 300);
        assert_eq!(buf[0], 0);
        assert_eq!(buf[255], 255);
        assert_eq!(buf[256], 0);
    }

    #[test]
    fn test_repeat_stream() {
        let mut stream = SyntheticStream::repeat(vec![1, 2, 3], 2);
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        assert_eq!(buf, vec![1, 2, 3, 1, 2, 3]);
    }

    #[test]
    fn test_random_stream_deterministic() {
        let mut s1 = SyntheticStream::random(50, 42);
        let mut s2 = SyntheticStream::random(50, 42);

        let mut b1 = Vec::new();
        let mut b2 = Vec::new();
        s1.read_to_end(&mut b1).unwrap();
        s2.read_to_end(&mut b2).unwrap();

        assert_eq!(b1, b2);
    }
}
