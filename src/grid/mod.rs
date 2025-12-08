use std::fmt::Display;

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
    pub position: (usize, usize),
}

impl Neighbor {
    pub fn new(direction: Direction, x: usize, y: usize) -> Self {
        Self {
            direction,
            position: (x, y),
        }
    }

    pub fn next<T>(self, grid: &[Vec<T>]) -> Option<Neighbor> {
        let Neighbor {
            direction,
            position: (x, y),
        } = self;

        match direction {
            Direction::Right => {
                if grid.get(y).and_then(|r| r.get(x + 1)).is_some() {
                    Some(Self::new(Direction::Right, x + 1, y))
                } else {
                    None
                }
            }
            Direction::Left => {
                if grid.get(y).is_some() && x > 0 {
                    Some(Self::new(Direction::Left, x - 1, y))
                } else {
                    None
                }
            }
            Direction::Up => {
                if y > 0 {
                    Some(Self::new(Direction::Up, x, y - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if grid.get(y + 1).and_then(|r| r.get(x)).is_some() {
                    Some(Self::new(Direction::Down, x, y + 1))
                } else {
                    None
                }
            }
            Direction::UpperRight => {
                if y > 0 && grid[y - 1].get(x + 1).is_some() {
                    Some(Self::new(Direction::UpperRight, x + 1, y - 1))
                } else {
                    None
                }
            }
            Direction::UpperLeft => {
                if y > 0 && x > 0 {
                    Some(Self::new(Direction::UpperLeft, x - 1, y - 1))
                } else {
                    None
                }
            }
            Direction::LowerRight => {
                if grid.get(y + 1).and_then(|r| r.get(x + 1)).is_some() {
                    Some(Self::new(Direction::LowerRight, x + 1, y + 1))
                } else {
                    None
                }
            }
            Direction::LowerLeft => {
                if grid.get(y + 1).is_some() && x > 0 {
                    Some(Self::new(Direction::LowerLeft, x - 1, y + 1))
                } else {
                    None
                }
            }
        }
    }
}

impl From<Neighbor> for (usize, usize) {
    fn from(value: Neighbor) -> Self {
        value.position
    }
}

pub fn neighbor_in_direction<T>(
    grid: &[Vec<T>],
    direction: Direction,
    x: usize,
    y: usize,
) -> Option<Neighbor> {
    match direction {
        Direction::Up => y.checked_sub(1).map(|y| Neighbor::new(direction, x, y)),
        Direction::Down => grid.get(y + 1).map(|_| Neighbor::new(direction, x, y + 1)),
        Direction::Left => x.checked_sub(1).map(|x| Neighbor::new(direction, x, y)),
        Direction::Right => grid[y]
            .get(x + 1)
            .map(|_| Neighbor::new(direction, x + 1, y)),
        Direction::UpperLeft => y
            .checked_sub(1)
            .filter(|_| x > 0)
            .map(|y| Neighbor::new(direction, x - 1, y)),
        Direction::UpperRight => y
            .checked_sub(1)
            .and_then(|y| grid[y].get(x + 1))
            .map(|_| Neighbor::new(direction, x + 1, y - 1)),
        Direction::LowerLeft => grid
            .get(y + 1)
            .filter(|_| x > 0)
            .map(|_| Neighbor::new(direction, x - 1, y + 1)),
        Direction::LowerRight => grid
            .get(y + 1)
            .and_then(|_| grid[y + 1].get(x + 1))
            .map(|_| Neighbor::new(direction, x + 1, y + 1)),
    }
}

pub fn neighbors<T>(grid: &[Vec<T>], x: usize, y: usize, include_diagonals: bool) -> Vec<Neighbor> {
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
        .filter_map(|d| neighbor_in_direction(grid, d, x, y))
        .collect()
}

pub fn print_grid<T, F, O>(grid: &[Vec<T>], mapper: F)
where
    F: Fn(&T) -> O,
    O: Display,
{
    for row in grid {
        for col in row {
            print!("{}", mapper(col));
        }
        println!();
    }
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
                Neighbor::new(Direction::Right, 1, 0),
                Neighbor::new(Direction::Down, 0, 1),
            ],
            neighbors(&grid, 0, 0, false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Right, 1, 0),
                Neighbor::new(Direction::Down, 0, 1),
                Neighbor::new(Direction::LowerRight, 1, 1),
            ],
            neighbors(&grid, 0, 0, true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Left, 4, 0),
                Neighbor::new(Direction::Right, 6, 0),
                Neighbor::new(Direction::Down, 5, 1),
            ],
            neighbors(&grid, 5, 0, false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Left, 4, 0),
                Neighbor::new(Direction::Right, 6, 0),
                Neighbor::new(Direction::Down, 5, 1),
                Neighbor::new(Direction::LowerLeft, 4, 1),
                Neighbor::new(Direction::LowerRight, 6, 1),
            ],
            neighbors(&grid, 5, 0, true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Left, 8, 0),
                Neighbor::new(Direction::Down, 9, 1),
            ],
            neighbors(&grid, 9, 0, false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Left, 8, 0),
                Neighbor::new(Direction::Down, 9, 1),
                Neighbor::new(Direction::LowerLeft, 8, 1),
            ],
            neighbors(&grid, 9, 0, true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Up, 0, 4),
                Neighbor::new(Direction::Down, 0, 6),
                Neighbor::new(Direction::Right, 1, 5),
            ],
            neighbors(&grid, 0, 5, false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Up, 0, 4),
                Neighbor::new(Direction::Down, 0, 6),
                Neighbor::new(Direction::Right, 1, 5),
                Neighbor::new(Direction::UpperRight, 1, 4),
                Neighbor::new(Direction::LowerRight, 1, 6),
            ],
            neighbors(&grid, 0, 5, true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Up, 0, 8),
                Neighbor::new(Direction::Right, 1, 9),
            ],
            neighbors(&grid, 0, 9, false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Up, 0, 8),
                Neighbor::new(Direction::Right, 1, 9),
                Neighbor::new(Direction::UpperRight, 1, 8),
            ],
            neighbors(&grid, 0, 9, true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Left, 3, 4),
                Neighbor::new(Direction::Up, 4, 3),
                Neighbor::new(Direction::Down, 4, 5),
                Neighbor::new(Direction::Right, 5, 4),
            ],
            neighbors(&grid, 4, 4, false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::UpperLeft, 3, 3),
                Neighbor::new(Direction::Left, 3, 4),
                Neighbor::new(Direction::LowerLeft, 3, 5),
                Neighbor::new(Direction::Up, 4, 3),
                Neighbor::new(Direction::Down, 4, 5),
                Neighbor::new(Direction::UpperRight, 5, 3),
                Neighbor::new(Direction::Right, 5, 4),
                Neighbor::new(Direction::LowerRight, 5, 5),
            ],
            neighbors(&grid, 4, 4, true),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::Up, 9, 8),
                Neighbor::new(Direction::Left, 8, 9),
            ],
            neighbors(&grid, 9, 9, false),
        );

        assert_eq_ignore_order(
            vec![
                Neighbor::new(Direction::UpperLeft, 8, 8),
                Neighbor::new(Direction::Up, 9, 8),
                Neighbor::new(Direction::Left, 8, 9),
            ],
            neighbors(&grid, 9, 9, true),
        );
    }
}
