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

```rust
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

let grid: Grid<i32> = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
].try_into()?;

// Dimension queries
grid.height(); // 2
grid.width();  // 3

// Bounds-checked access
grid.get(Point::new(0, 0));     // Some(&1)
grid.get(Point::new(99, 99));   // None

// Direct indexing (panics on out-of-bounds)
let val = grid[Point::new(1, 1)]; // 5

// Mutable access
grid[Point::new(1, 1)] = 42;

// Iteration (yields &Vec<T> rows)
for row in &grid { /* ... */ }
for row in &mut grid { /* ... */ }
```

`Grid<T>` implements `Deref<Target = [Vec<T>]>` and `DerefMut`, so all slice methods are available directly.

#### `Direction`

Eight-directional enum for navigating grids.

```rust
pub enum Direction {
    Up, Down, Left, Right,
    UpperRight, UpperLeft,
    LowerRight, LowerLeft,
}
```

Converts to a Unicode arrow character via `char::from(direction)`.

#### `Neighbor`

A `(Direction, Point)` pair representing an adjacent cell.

```rust
pub struct Neighbor {
    pub direction: Direction,
    pub position: Point,
}
```

`Neighbor::next(&grid)` — returns the next `Neighbor` continuing in the same direction, or `None` if out of bounds.

Converts to `Point` or `(usize, usize)` via `From`.

#### Free functions

```rust
// Single neighbor lookup
neighbor_in_direction(&grid, Direction::Right, Point::new(2, 3)) -> Option<Neighbor>

// All neighbors of a position
// include_diagonals=false → up to 4 neighbors (cardinal)
// include_diagonals=true  → up to 8 neighbors (cardinal + diagonal)
neighbors(&grid, Point::new(2, 3), true) -> Vec<Neighbor>

// Print a grid to any writer (e.g. stdout, a String buffer)
print_grid(&grid, |cell| *cell, &mut std::io::stdout())?;
```

---

### `math`

Mathematical utilities and coordinate types.

#### `math::two_dimensional::Point`

A 2D point with `usize` coordinates.

```rust
use aoc_util::math::two_dimensional::Point;

let p = Point::new(3, 7);
p.x; // 3
p.y; // 7

p.distance(&Point::new(0, 0)); // f64 Euclidean distance

// Parse from "x,y"
let p: Point = "3,7".parse()?;

// Display: "(3, 7)"
println!("{p}");

// Destructure
let (x, y): (usize, usize) = p.into();
```

#### `math::three_dimensional::Point`

A 3D point with `usize` coordinates.

```rust
use aoc_util::math::three_dimensional::Point;

let p = Point::new(1, 2, 3);
p.distance(&Point::new(0, 0, 0)); // f64 Euclidean distance

// Parse from "x,y,z"
let p: Point = "1,2,3".parse()?;

// Destructure
let (x, y, z): (usize, usize, usize) = p.into();
```

#### `MinMax<T>`

Collect the minimum and maximum of an iterator in one pass.

```rust
use aoc_util::math::MinMax;

let values = vec![3, 1, 4, 1, 5, 9];

// From a borrowing iterator
let mm: MinMax<i32> = values.iter().collect();
mm.min; // Some(1)
mm.max; // Some(9)

// From a consuming iterator
let mm: MinMax<i32> = values.into_iter().collect();
```

`MinMax` requires `T: Ord + Copy`.

#### `greatest_common_divisor(a, b) -> T`

Computes the GCD of two values using the Euclidean algorithm. Generic over any integer type
(`Ord + Copy + Rem + From<u8>`).

```rust
use aoc_util::math::greatest_common_divisor;

greatest_common_divisor(48u64, 18u64); // 6
```

#### `least_common_multiple(a, b) -> T`

Computes the LCM of two values without overflow (uses `a / gcd(a,b) * b`). Generic over any integer type
(`Ord + Copy + Rem + Div + Mul + From<u8>`).

```rust
use aoc_util::math::least_common_multiple;

least_common_multiple(48u64, 18u64); // 144
```

---

### `logging`

Internal logging setup (used automatically by `init()` and `init_test()`).

#### `init_test_logger()`

Initializes a `Trace`-level logger targeting stdout, suitable for use in `#[test]` functions. Safe to call multiple
times (subsequent calls are no-ops).

```rust
#[test]
fn my_test() -> anyhow::Result<()> {
    let lines = aoc_util::init_test()?;
    // ...
    Ok(())
}
```
