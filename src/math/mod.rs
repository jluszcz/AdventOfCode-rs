use std::cmp::{max, min};
use std::ops::{Div, Mul, Rem};

#[derive(Debug, Copy, Clone, Default)]
pub struct MinMax<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

// For owned values
impl<T: Ord + Copy> FromIterator<T> for MinMax<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut min_val = None;
        let mut max_val = None;

        for i in iter {
            min_val = Some(match min_val {
                None => i,
                Some(m) => min(m, i),
            });
            max_val = Some(match max_val {
                None => i,
                Some(m) => max(m, i),
            });
        }

        MinMax {
            min: min_val,
            max: max_val,
        }
    }
}

// For references - enables iter() instead of into_iter()
impl<'a, T: Ord + Copy + 'a> FromIterator<&'a T> for MinMax<T> {
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let mut min_val = None;
        let mut max_val = None;

        for &i in iter {
            min_val = Some(match min_val {
                None => i,
                Some(m) => min(m, i),
            });
            max_val = Some(match max_val {
                None => i,
                Some(m) => max(m, i),
            });
        }

        MinMax {
            min: min_val,
            max: max_val,
        }
    }
}

pub fn greatest_common_divisor<T>(mut a: T, mut b: T) -> T
where
    T: Ord + Copy + Rem<Output = T> + From<u8>,
{
    let zero = T::from(0);

    // Ensure a >= b
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    // Euclidean algorithm (iterative)
    while b != zero {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

pub fn least_common_multiple<T>(a: T, b: T) -> T
where
    T: Ord + Copy + Rem<Output = T> + Div<Output = T> + Mul<Output = T> + From<u8>,
{
    a / greatest_common_divisor(a, b) * b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_least_common_multiple() {
        assert_eq!(144, least_common_multiple(48, 18));
        assert_eq!(144u32, least_common_multiple(48u32, 18u32));
        assert_eq!(144i64, least_common_multiple(48i64, 18i64));

        // Test case where a * b would overflow but LCM fits
        // gcd(10_000_000_000, 20_000_000_000) = 10_000_000_000
        // LCM = 20_000_000_000 (fits in u64)
        // but a * b = 200_000_000_000_000_000_000 (overflows u64)
        assert_eq!(
            20_000_000_000u64,
            least_common_multiple(10_000_000_000u64, 20_000_000_000u64)
        );
    }

    #[test]
    fn test_greatest_common_divisor() {
        assert_eq!(6, greatest_common_divisor(48, 18));
        assert_eq!(6u32, greatest_common_divisor(48u32, 18u32));
        assert_eq!(6i64, greatest_common_divisor(48i64, 18i64));
    }

    #[test]
    fn test_min_max() {
        // Test with i32/iter
        let values_i32: Vec<i32> = vec![-10, 50, 0, 100, -50];
        let min_max_i32 = values_i32.iter().collect::<MinMax<i32>>();
        assert_eq!(min_max_i32.min, Some(-50));
        assert_eq!(min_max_i32.max, Some(100));

        // Test with u64/into_iter
        let values_u64: Vec<u64> = vec![1000, 2000, 500];
        let min_max_u64 = values_u64.into_iter().collect::<MinMax<u64>>();
        assert_eq!(min_max_u64.min, Some(500));
        assert_eq!(min_max_u64.max, Some(2000));
    }
}
