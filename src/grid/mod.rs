use std::fmt::Display;
use std::io::Write;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpperRight,
    UpperLeft,
    LowerRight,
    LowerLeft,
}

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => '↑',
            Direction::Down => '↓',
            Direction::Left => '←',
            Direction::Right => '→',
            Direction::UpperLeft => '↖',
            Direction::UpperRight => '↗',
            Direction::LowerLeft => '↙',
            Direction::LowerRight => '↘',
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Neighbor {
    pub direction: Direction,
    pub position: Point,
}

impl Neighbor {
    pub fn new(direction: Direction, position: Point) -> Self {
        Self {
            direction,
            position,
        }
    }

    pub fn next<T>(self, grid: &[Vec<T>]) -> Option<Neighbor> {
        let Neighbor {
            direction,
            position,
        } = self;
        let x = position.x;
        let y = position.y;

        match direction {
            Direction::Right => {
                if grid.get(y).and_then(|r| r.get(x + 1)).is_some() {
                    Some(Self::new(Direction::Right, Point::new(x + 1, y)))
                } else {
                    None
                }
            }
            Direction::Left => {
                if grid.get(y).is_some() && x > 0 {
                    Some(Self::new(Direction::Left, Point::new(x - 1, y)))
                } else {
                    None
                }
            }
            Direction::Up => {
                if y > 0 {
                    Some(Self::new(Direction::Up, Point::new(x, y - 1)))
                } else {
                    None
                }
            }
            Direction::Down => {
                if grid.get(y + 1).and_then(|r| r.get(x)).is_some() {
                    Some(Self::new(Direction::Down, Point::new(x, y + 1)))
                } else {
                    None
                }
            }
            Direction::UpperRight => {
                if y > 0 && grid[y - 1].get(x + 1).is_some() {
                    Some(Self::new(Direction::UpperRight, Point::new(x + 1, y - 1)))
                } else {
                    None
                }
            }
            Direction::UpperLeft => {
                if y > 0 && x > 0 {
                    Some(Self::new(Direction::UpperLeft, Point::new(x - 1, y - 1)))
                } else {
                    None
                }
            }
            Direction::LowerRight => {
                if grid.get(y + 1).and_then(|r| r.get(x + 1)).is_some() {
                    Some(Self::new(Direction::LowerRight, Point::new(x + 1, y + 1)))
                } else {
                    None
                }
            }
            Direction::LowerLeft => {
                if grid.get(y + 1).is_some() && x > 0 {
                    Some(Self::new(Direction::LowerLeft, Point::new(x - 1, y + 1)))
                } else {
                    None
                }
            }
        }
    }
}

impl From<Neighbor> for Point {
    fn from(value: Neighbor) -> Self {
        value.position
    }
}

impl From<Neighbor> for (usize, usize) {
    fn from(value: Neighbor) -> Self {
        value.position.into()
    }
}

pub fn neighbor_in_direction<T>(
    grid: &[Vec<T>],
    direction: Direction,
    position: Point,
) -> Option<Neighbor> {
    let x = position.x;
    let y = position.y;

    match direction {
        Direction::Up => y
            .checked_sub(1)
            .map(|y| Neighbor::new(direction, Point::new(x, y))),
        Direction::Down => grid
            .get(y + 1)
            .map(|_| Neighbor::new(direction, Point::new(x, y + 1))),
        Direction::Left => x
            .checked_sub(1)
            .map(|x| Neighbor::new(direction, Point::new(x, y))),
        Direction::Right => grid[y]
            .get(x + 1)
            .map(|_| Neighbor::new(direction, Point::new(x + 1, y))),
        Direction::UpperLeft => y
            .checked_sub(1)
            .filter(|_| x > 0)
            .map(|y| Neighbor::new(direction, Point::new(x - 1, y))),
        Direction::UpperRight => y
            .checked_sub(1)
            .and_then(|y| grid[y].get(x + 1))
            .map(|_| Neighbor::new(direction, Point::new(x + 1, y - 1))),
        Direction::LowerLeft => grid
            .get(y + 1)
            .filter(|_| x > 0)
            .map(|_| Neighbor::new(direction, Point::new(x - 1, y + 1))),
        Direction::LowerRight => grid
            .get(y + 1)
            .and_then(|_| grid[y + 1].get(x + 1))
            .map(|_| Neighbor::new(direction, Point::new(x + 1, y + 1))),
    }
}

pub fn neighbors<T>(grid: &[Vec<T>], position: Point, include_diagonals: bool) -> Vec<Neighbor> {
    let mut directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    if include_diagonals {
        directions.extend_from_slice(&[
            Direction::UpperLeft,
            Direction::UpperRight,
            Direction::LowerLeft,
            Direction::LowerRight,
        ]);
    }

    directions
        .into_iter()
        .filter_map(|d| neighbor_in_direction(grid, d, position))
        .collect()
}

pub fn print_grid<T, F, O, W>(grid: &[Vec<T>], mapper: F, writer: &mut W) -> std::io::Result<()>
where
    F: Fn(&T) -> O,
    O: Display,
    W: Write,
{
    for row in grid {
        for col in row {
            write!(writer, "{}", mapper(col))?;
        }
        writeln!(writer)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let grid = vec![vec![0; 10]; 10];

        fn assert_eq_ignore_order(mut expected: Vec<Neighbor>, mut neighbors: Vec<Neighbor>) {
            expected.sort_unstable();
            neighbors.sort_unstable();
            assert_eq!(expected, neighbors);
        }

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Right, Point::new(1, 0)),
                Neighbor::new(Direction::Down, Point::new(0, 1)),
            ],
            neighbors(&grid, Point::new(0, 0), false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Right, Point::new(1, 0)),
                Neighbor::new(Direction::Down, Point::new(0, 1)),
                Neighbor::new(Direction::LowerRight, Point::new(1, 1)),
            ],
            neighbors(&grid, Point::new(0, 0), true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Left, Point::new(4, 0)),
                Neighbor::new(Direction::Right, Point::new(6, 0)),
                Neighbor::new(Direction::Down, Point::new(5, 1)),
            ],
            neighbors(&grid, Point::new(5, 0), false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Left, Point::new(4, 0)),
                Neighbor::new(Direction::Right, Point::new(6, 0)),
                Neighbor::new(Direction::Down, Point::new(5, 1)),
                Neighbor::new(Direction::LowerLeft, Point::new(4, 1)),
                Neighbor::new(Direction::LowerRight, Point::new(6, 1)),
            ],
            neighbors(&grid, Point::new(5, 0), true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Left, Point::new(8, 0)),
                Neighbor::new(Direction::Down, Point::new(9, 1)),
            ],
            neighbors(&grid, Point::new(9, 0), false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Left, Point::new(8, 0)),
                Neighbor::new(Direction::Down, Point::new(9, 1)),
                Neighbor::new(Direction::LowerLeft, Point::new(8, 1)),
            ],
            neighbors(&grid, Point::new(9, 0), true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Up, Point::new(0, 4)),
                Neighbor::new(Direction::Down, Point::new(0, 6)),
                Neighbor::new(Direction::Right, Point::new(1, 5)),
            ],
            neighbors(&grid, Point::new(0, 5), false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Up, Point::new(0, 4)),
                Neighbor::new(Direction::Down, Point::new(0, 6)),
                Neighbor::new(Direction::Right, Point::new(1, 5)),
                Neighbor::new(Direction::UpperRight, Point::new(1, 4)),
                Neighbor::new(Direction::LowerRight, Point::new(1, 6)),
            ],
            neighbors(&grid, Point::new(0, 5), true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Up, Point::new(0, 8)),
                Neighbor::new(Direction::Right, Point::new(1, 9)),
            ],
            neighbors(&grid, Point::new(0, 9), false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Up, Point::new(0, 8)),
                Neighbor::new(Direction::Right, Point::new(1, 9)),
                Neighbor::new(Direction::UpperRight, Point::new(1, 8)),
            ],
            neighbors(&grid, Point::new(0, 9), true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Left, Point::new(3, 4)),
                Neighbor::new(Direction::Up, Point::new(4, 3)),
                Neighbor::new(Direction::Down, Point::new(4, 5)),
                Neighbor::new(Direction::Right, Point::new(5, 4)),
            ],
            neighbors(&grid, Point::new(4, 4), false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::UpperLeft, Point::new(3, 3)),
                Neighbor::new(Direction::Left, Point::new(3, 4)),
                Neighbor::new(Direction::LowerLeft, Point::new(3, 5)),
                Neighbor::new(Direction::Up, Point::new(4, 3)),
                Neighbor::new(Direction::Down, Point::new(4, 5)),
                Neighbor::new(Direction::UpperRight, Point::new(5, 3)),
                Neighbor::new(Direction::Right, Point::new(5, 4)),
                Neighbor::new(Direction::LowerRight, Point::new(5, 5)),
            ],
            neighbors(&grid, Point::new(4, 4), true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Up, Point::new(9, 8)),
                Neighbor::new(Direction::Left, Point::new(8, 9)),
            ],
            neighbors(&grid, Point::new(9, 9), false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::UpperLeft, Point::new(8, 8)),
                Neighbor::new(Direction::Up, Point::new(9, 8)),
                Neighbor::new(Direction::Left, Point::new(8, 9)),
            ],
            neighbors(&grid, Point::new(9, 9), true),
        );
    }

    #[test]
    fn test_print_grid_to_writer() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let mut buffer = Vec::new();
        print_grid(&grid, |x| *x, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "123\n456\n789\n");
    }
}
