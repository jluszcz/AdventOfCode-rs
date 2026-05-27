use anyhow::anyhow;
use anyhow::bail;
use std::cmp::{max, min};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

pub trait ToF64: Copy {
    fn to_f64(self) -> f64;
}

macro_rules! impl_to_f64 {
    ($($t:ty),*) => {
        $(impl ToF64 for $t {
            fn to_f64(self) -> f64 { self as f64 }
        })*
    };
}

impl_to_f64!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

pub mod two_dimensional {
    use super::*;

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    pub struct Point<T = usize> {
        pub x: T,
        pub y: T,
    }

    impl<T> Point<T> {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: ToF64> Point<T> {
        pub fn distance(&self, other: &Self) -> f64 {
            let dx = self.x.to_f64() - other.x.to_f64();
            let dy = self.y.to_f64() - other.y.to_f64();
            (dx * dx + dy * dy).sqrt()
        }
    }

    impl<T> Point<T>
    where
        T: Copy + Ord + Sub<Output = T> + Add<Output = T>,
    {
        pub fn manhattan_distance(&self, other: &Self) -> T {
            let dx = if self.x >= other.x {
                self.x - other.x
            } else {
                other.x - self.x
            };
            let dy = if self.y >= other.y {
                self.y - other.y
            } else {
                other.y - self.y
            };
            dx + dy
        }
    }

    impl<T> From<Point<T>> for (T, T) {
        fn from(value: Point<T>) -> Self {
            (value.x, value.y)
        }
    }

    impl<T: Display> Display for Point<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl<T: Display> Debug for Point<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self}")
        }
    }

    impl<T> FromStr for Point<T>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> anyhow::Result<Self> {
            let (x, y) = s.split_once(',').ok_or_else(|| anyhow!("Invalid point"))?;
            Ok(Self::new(x.parse()?, y.parse()?))
        }
    }
}

pub mod three_dimensional {
    use super::*;

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    pub struct Point<T = usize> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl<T> Point<T> {
        pub fn new(x: T, y: T, z: T) -> Self {
            Self { x, y, z }
        }
    }

    impl<T: ToF64> Point<T> {
        pub fn distance(&self, other: &Self) -> f64 {
            let dx = self.x.to_f64() - other.x.to_f64();
            let dy = self.y.to_f64() - other.y.to_f64();
            let dz = self.z.to_f64() - other.z.to_f64();
            (dx * dx + dy * dy + dz * dz).sqrt()
        }
    }

    impl<T> Point<T>
    where
        T: Copy + Ord + Sub<Output = T> + Add<Output = T>,
    {
        pub fn manhattan_distance(&self, other: &Self) -> T {
            let dx = if self.x >= other.x {
                self.x - other.x
            } else {
                other.x - self.x
            };
            let dy = if self.y >= other.y {
                self.y - other.y
            } else {
                other.y - self.y
            };
            let dz = if self.z >= other.z {
                self.z - other.z
            } else {
                other.z - self.z
            };
            dx + dy + dz
        }
    }

    impl<T> From<Point<T>> for (T, T, T) {
        fn from(value: Point<T>) -> Self {
            (value.x, value.y, value.z)
        }
    }

    impl<T: Display> Display for Point<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {})", self.x, self.y, self.z)
        }
    }

    impl<T: Display> Debug for Point<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self}")
        }
    }

    impl<T> FromStr for Point<T>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> anyhow::Result<Self> {
            let parts = s.split(',').collect::<Vec<_>>();
            if parts.len() != 3 {
                bail!("Invalid point");
            }

            let x = parts[0].parse()?;
            let y = parts[1].parse()?;
            let z = parts[2].parse()?;

            Ok(Self::new(x, y, z))
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct MinMax<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

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

impl<'a, T: Ord + Copy + 'a> FromIterator<&'a T> for MinMax<T> {
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        iter.into_iter().copied().collect()
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
mod tests {
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

    #[test]
    fn test_manhattan_distance_2d() {
        let p1 = two_dimensional::Point::new(0usize, 0);
        let p2 = two_dimensional::Point::new(3usize, 4);
        assert_eq!(p1.manhattan_distance(&p2), 7);
        assert_eq!(p2.manhattan_distance(&p1), 7);

        // Mixed-sign coordinates via signed type
        let p3 = two_dimensional::Point::new(-5i32, 3);
        let p4 = two_dimensional::Point::new(2i32, -1);
        assert_eq!(p3.manhattan_distance(&p4), 11);
    }

    #[test]
    fn test_manhattan_distance_3d() {
        let p1 = three_dimensional::Point::new(0usize, 0, 0);
        let p2 = three_dimensional::Point::new(1usize, 2, 3);
        assert_eq!(p1.manhattan_distance(&p2), 6);

        let p3 = three_dimensional::Point::new(-1i64, -2, -3);
        let p4 = three_dimensional::Point::new(1i64, 2, 3);
        assert_eq!(p3.manhattan_distance(&p4), 12);
    }

    #[test]
    fn test_point_2d_signed_parse() {
        let p: two_dimensional::Point<i64> = "-5,3".parse().unwrap();
        assert_eq!(p.x, -5);
        assert_eq!(p.y, 3);
    }

    #[test]
    fn test_point_3d_signed_parse() {
        let p: three_dimensional::Point<i64> = "-1,2,-3".parse().unwrap();
        assert_eq!(p.x, -1);
        assert_eq!(p.y, 2);
        assert_eq!(p.z, -3);
    }

    #[test]
    fn test_point_distance() {
        let p1 = two_dimensional::Point::new(0usize, 0);
        let p2 = two_dimensional::Point::new(3usize, 4);
        assert_eq!(p1.distance(&p2), 5.0);

        let p3 = three_dimensional::Point::new(0i64, 0, 0);
        let p4 = three_dimensional::Point::new(2i64, 3, 6);
        assert_eq!(p4.distance(&p3), 7.0);
    }
}
