# AdventOfCode-rs
Rust utilities for Advent of Code

## Overview

`aoc_util` is a library crate providing common utilities for solving [Advent of Code](https://adventofcode.com/) puzzles
in Rust. It handles  boilerplate like CLI argument parsing, logging setup, and input file loading, and provides data
structures for grid-based and math-heavy problems.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
aoc_util = { path = "../AdventOfCode-rs" }
```

A typical puzzle entry point looks like:

```rust,no_run
fn main() -> anyhow::Result<()> {
    let lines = aoc_util::init()?;
    // lines is Vec<String> from input/input or input/example
    Ok(())
}
```

Input files are expected at:
- `input/input` — actual puzzle input (default)
- `input/example` — example/test input

### CLI Flags

`init()` installs a CLI with two flags:

| Flag | Default | Description |
|------|---------|-------------|
| `-i`, `--input` | `actual` | Input type: `actual` or `test` |
| `-v`, `--verbose` | off | Increase log verbosity |

Log levels by combination:

| Input | Verbose | Log Level |
|-------|---------|-----------|
| actual | false | Info |
| actual | true | Debug |
| test | false | Debug |
| test | true | Trace |

---

## Modules

### `grid`

Data structures and utilities for 2D grid problems.

#### `Grid<T>`

A rectangular 2D grid backed by `Vec<Vec<T>>`. All rows must be the same length (enforced at construction).

```rust
use aoc_util::grid::Grid;
use aoc_util::math::two_dimensional::Point;

let mut grid: Grid<i32> = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
].try_into().unwrap();

// Dimension queries
assert_eq!(grid.height(), 2);
assert_eq!(grid.width(), 3);

// Bounds-checked access
assert_eq!(grid.get(Point::new(0, 0)), Some(&1));
assert_eq!(grid.get(Point::new(99, 99)), None);

// Direct indexing (panics on out-of-bounds). Point::new(x, y) is column-major:
// x indexes within a row, y selects the row.
assert_eq!(grid[Point::new(1, 1)], 5);

// Mutable access
grid[Point::new(1, 1)] = 42;
assert_eq!(grid[Point::new(1, 1)], 42);

// Iteration yields row slices: &[T], and &mut [T] for the mutable form
for row in &grid {
    let _: &[i32] = row;
}
for row in &mut grid {
    let _: &mut [i32] = row;
}
```

`Grid<T>` implements `Deref<Target = [Vec<T>]>` and `DerefMut`, so all slice methods are available directly.

#### `Direction`

Eight-directional enum for navigating grids.

```text
pub enum Direction {
    Up, Down, Left, Right,
    UpperRight, UpperLeft,
    LowerRight, LowerLeft,
}
```

Converts to a Unicode arrow character via `char::from(direction)`.

#### `Neighbor`

A `(Direction, Point)` pair representing an adjacent cell.

```text
pub struct Neighbor {
    pub direction: Direction,
    pub position: Point,
}
```

`Neighbor::next(&grid)` — returns the next `Neighbor` continuing in the same direction, or `None` if out of bounds.

Converts to `Point` or `(usize, usize)` via `From`.

#### Free functions

```rust
use aoc_util::grid::{Direction, Grid, neighbor_in_direction, neighbors, print_grid};
use aoc_util::math::two_dimensional::Point;

let grid: Grid<char> = vec![
    vec!['a', 'b', 'c'],
    vec!['d', 'e', 'f'],
].try_into().unwrap();

// Single neighbor lookup
let right = neighbor_in_direction(&grid, Direction::Right, Point::new(0, 0));
assert_eq!(right.map(|n| n.position), Some(Point::new(1, 0)));

// All neighbors of a position:
//   include_diagonals = false → up to 4 (cardinal)
//   include_diagonals = true  → up to 8 (cardinal + diagonal)
assert_eq!(neighbors(&grid, Point::new(0, 0), false).len(), 2);
assert_eq!(neighbors(&grid, Point::new(0, 0), true).len(), 3);

// Print a grid to any writer (e.g. stdout, a String buffer)
let mut buffer = Vec::new();
print_grid(&grid, |cell| *cell, &mut buffer).unwrap();
assert_eq!(String::from_utf8(buffer).unwrap(), "abc\ndef\n");
```

---

### `math`

Mathematical utilities and coordinate types.

#### `math::two_dimensional::Point<T = usize>`

A 2D point, generic over its coordinate type. `usize` is the default, which is what grid code wants; instantiate it
with a signed type when coordinates can go negative.

```rust
use aoc_util::math::two_dimensional::Point;

let p = Point::new(3, 7);
assert_eq!(p.x, 3);
assert_eq!(p.y, 7);

// Euclidean distance, as f64. Requires T: ToF64 (implemented for every
// primitive integer and float).
assert_eq!(p.distance(&Point::new(0, 4)), 3.0_f64.hypot(3.0));

// Manhattan distance, in T. Requires T: Copy + Ord + Sub + Add.
assert_eq!(p.manhattan_distance(&Point::new(0, 4)), 6);

// Parse from "x,y"
let p: Point = "3,7".parse().unwrap();

// Display: "(3, 7)"
assert_eq!(p.to_string(), "(3, 7)");

// Destructure
let (x, y): (usize, usize) = p.into();
assert_eq!((x, y), (3, 7));

// Signed coordinates
let below: Point<i64> = Point::new(-2, -5);
assert_eq!(below.manhattan_distance(&Point::new(1, -1)), 7);
```

#### `math::three_dimensional::Point<T = usize>`

The 3D counterpart, with the same generic parameter and the same two distance methods.

```rust
use aoc_util::math::three_dimensional::Point;

let p = Point::new(1, 2, 3);
assert_eq!(p.distance(&Point::new(1, 2, 0)), 3.0);
assert_eq!(p.manhattan_distance(&Point::new(0, 0, 0)), 6);

// Parse from "x,y,z"
let p: Point = "1,2,3".parse().unwrap();

// Display: "(1, 2, 3)"
assert_eq!(p.to_string(), "(1, 2, 3)");

// Destructure
let (x, y, z): (usize, usize, usize) = p.into();
assert_eq!((x, y, z), (1, 2, 3));
```

#### `MinMax<T>`

Collect the minimum and maximum of an iterator in one pass.

```rust
use aoc_util::math::MinMax;

let values = vec![3, 1, 4, 1, 5, 9];

// From a borrowing iterator
let mm: MinMax<i32> = values.iter().collect();
assert_eq!(mm.min, Some(1));
assert_eq!(mm.max, Some(9));

// From a consuming iterator
let mm: MinMax<i32> = values.into_iter().collect();
assert_eq!(mm.max, Some(9));

// Both fields are None for an empty iterator
let empty: MinMax<i32> = Vec::<i32>::new().into_iter().collect();
assert_eq!((empty.min, empty.max), (None, None));
```

`MinMax` requires `T: Ord + Copy`.

#### `greatest_common_divisor(a, b) -> T`

Computes the GCD of two values using the Euclidean algorithm. Generic over any integer type
(`Ord + Copy + Rem + From<u8>`).

```rust
use aoc_util::math::greatest_common_divisor;

assert_eq!(greatest_common_divisor(48u64, 18u64), 6);
```

Defined for unsigned values. A negative argument yields a negative result, which is not the usual GCD contract.

#### `least_common_multiple(a, b) -> T`

Computes the LCM of two values without overflow (uses `a / gcd(a,b) * b`). Generic over any integer type
(`Ord + Copy + Rem + Div + Mul + From<u8>`).

```rust
use aoc_util::math::least_common_multiple;

assert_eq!(least_common_multiple(48u64, 18u64), 144);
```

`least_common_multiple(0, 0)` panics — it divides by `gcd(0, 0)`, which is zero.

---

### `logging`

Internal logging setup (used automatically by `init()` and `init_test()`).

#### `init_test_logger()`

Initializes a `Trace`-level logger targeting stdout, suitable for use in `#[test]` functions. Safe to call multiple
times (subsequent calls are no-ops).

```rust,no_run
#[test]
fn my_test() -> anyhow::Result<()> {
    let lines = aoc_util::init_test()?;
    // ...
    Ok(())
}
```

---

## Development

```sh
cargo build
cargo test        # includes the examples in this README as doc-tests
cargo fmt --check
cargo clippy --all-targets -- -D warnings
pre-commit install    # once, to enable the commit hooks
```

CI runs the same four cargo commands via `jluszcz/github-utils`. The examples above are compiled as doc-tests, so an
API change that this README does not reflect fails the build.
