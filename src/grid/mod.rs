use crate::math::two_dimensional::Point;
use anyhow::{Result, bail};
use std::fmt::{Debug, Display};
use std::io::Write;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0.first().map_or(0, |row| row.len())
    }

    pub fn get(&self, position: Point) -> Option<&T> {
        self.0.get(position.y)?.get(position.x)
    }

    pub fn get_mut(&mut self, position: Point) -> Option<&mut T> {
        self.0.get_mut(position.y)?.get_mut(position.x)
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = anyhow::Error;

    fn try_from(data: Vec<Vec<T>>) -> Result<Self> {
        let mut row_len = None;
        for row in data.iter() {
            match row_len {
                None => row_len = Some(row.len()),
                Some(len) if len != row.len() => bail!("Rows must be the same length"),
                _ => (),
            }
        }

        Ok(Self(data))
    }
}

impl<T> Deref for Grid<T> {
    type Target = [Vec<T>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self.0[point.y][point.x]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.0[point.y][point.x]
    }
}

pub struct Rows<'a, T> {
    inner: std::slice::Iter<'a, Vec<T>>,
}

impl<'a, T> Iterator for Rows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Vec::as_slice)
    }
}

pub struct RowsMut<'a, T> {
    inner: std::slice::IterMut<'a, Vec<T>>,
}

impl<'a, T> Iterator for RowsMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Vec::as_mut_slice)
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a [T];
    type IntoIter = Rows<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Rows {
            inner: self.0.iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut [T];
    type IntoIter = RowsMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        RowsMut {
            inner: self.0.iter_mut(),
        }
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

impl Direction {
    pub fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpperLeft => (-1, -1),
            Direction::UpperRight => (1, -1),
            Direction::LowerLeft => (-1, 1),
            Direction::LowerRight => (1, 1),
        }
    }
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

    pub fn next<T>(self, grid: &Grid<T>) -> Option<Neighbor> {
        neighbor_in_direction(grid, self.direction, self.position)
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
    grid: &Grid<T>,
    direction: Direction,
    position: Point,
) -> Option<Neighbor> {
    let (dx, dy) = direction.delta();
    let x = position.x.checked_add_signed(dx)?;
    let y = position.y.checked_add_signed(dy)?;
    let new_position = Point::new(x, y);
    grid.get(new_position)
        .map(|_| Neighbor::new(direction, new_position))
}

pub fn neighbors<T>(grid: &Grid<T>, position: Point, include_diagonals: bool) -> Vec<Neighbor> {
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

pub fn print_grid<T, F, O, W>(grid: &Grid<T>, mapper: F, writer: &mut W) -> std::io::Result<()>
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
    fn test_neighbors() -> Result<()> {
        let grid = Grid::try_from(vec![vec![0; 10]; 10])?;

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

        Ok(())
    }

    #[test]
    fn test_neighbor_next() -> Result<()> {
        let grid = Grid::try_from(vec![vec![0; 5]; 5])?;

        // Step right one cell
        let n = Neighbor::new(Direction::Right, Point::new(0, 2));
        let next = n.next(&grid).expect("right step inside grid");
        assert_eq!(next.position, Point::new(1, 2));
        assert_eq!(next.direction, Direction::Right);

        // Step off the right edge
        let edge = Neighbor::new(Direction::Right, Point::new(4, 2));
        assert!(edge.next(&grid).is_none());

        // Step off the top edge (underflow)
        let top = Neighbor::new(Direction::Up, Point::new(2, 0));
        assert!(top.next(&grid).is_none());

        // Step off the left edge (underflow)
        let left = Neighbor::new(Direction::Left, Point::new(0, 2));
        assert!(left.next(&grid).is_none());

        // Diagonal step into the grid
        let diag = Neighbor::new(Direction::UpperLeft, Point::new(2, 2));
        let next = diag.next(&grid).expect("upper-left step inside grid");
        assert_eq!(next.position, Point::new(1, 1));
        assert_eq!(next.direction, Direction::UpperLeft);

        // Diagonal step off the corner
        let corner = Neighbor::new(Direction::UpperLeft, Point::new(0, 0));
        assert!(corner.next(&grid).is_none());

        // Walk across a row by repeatedly calling next
        let mut cur = Neighbor::new(Direction::Right, Point::new(0, 0));
        let mut visited = vec![cur.position];
        while let Some(n) = cur.next(&grid) {
            visited.push(n.position);
            cur = n;
        }
        assert_eq!(
            visited,
            (0..5).map(|x| Point::new(x, 0)).collect::<Vec<_>>()
        );

        Ok(())
    }

    #[test]
    fn test_direction_delta() {
        assert_eq!(Direction::Up.delta(), (0, -1));
        assert_eq!(Direction::Down.delta(), (0, 1));
        assert_eq!(Direction::Left.delta(), (-1, 0));
        assert_eq!(Direction::Right.delta(), (1, 0));
        assert_eq!(Direction::UpperLeft.delta(), (-1, -1));
        assert_eq!(Direction::UpperRight.delta(), (1, -1));
        assert_eq!(Direction::LowerLeft.delta(), (-1, 1));
        assert_eq!(Direction::LowerRight.delta(), (1, 1));
    }

    #[test]
    fn test_print_grid_to_writer() -> Result<()> {
        let grid = Grid::try_from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])?;

        let mut buffer = Vec::new();
        print_grid(&grid, |x| *x, &mut buffer)?;

        let output = String::from_utf8(buffer)?;
        assert_eq!(output, "123\n456\n789\n");

        Ok(())
    }

    #[test]
    fn test_grid_index_with_point() -> Result<()> {
        let grid = Grid::try_from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])?;

        assert_eq!(grid[Point::new(0, 0)], 1);
        assert_eq!(grid[Point::new(1, 0)], 2);
        assert_eq!(grid[Point::new(2, 0)], 3);
        assert_eq!(grid[Point::new(0, 1)], 4);
        assert_eq!(grid[Point::new(1, 1)], 5);
        assert_eq!(grid[Point::new(2, 1)], 6);
        assert_eq!(grid[Point::new(0, 2)], 7);
        assert_eq!(grid[Point::new(1, 2)], 8);
        assert_eq!(grid[Point::new(2, 2)], 9);

        Ok(())
    }

    #[test]
    fn test_grid_index_mut_with_point() -> Result<()> {
        let mut grid = Grid::try_from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])?;

        grid[Point::new(1, 1)] = 42;
        assert_eq!(grid[Point::new(1, 1)], 42);

        grid[Point::new(0, 0)] = 100;
        assert_eq!(grid[Point::new(0, 0)], 100);

        Ok(())
    }

    #[test]
    fn test_grid_deref_with_existing_functions() -> Result<()> {
        let grid = Grid::try_from(vec![vec![0; 10]; 10])?;

        // Grid should work with existing functions via Deref
        let ns = neighbors(&grid, Point::new(5, 5), false);
        assert_eq!(ns.len(), 4);

        let ns_diag = neighbors(&grid, Point::new(5, 5), true);
        assert_eq!(ns_diag.len(), 8);

        Ok(())
    }

    #[test]
    fn test_grid_dimensions() -> Result<()> {
        let grid = Grid::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]])?;
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.width(), 3);

        let empty_grid: Grid<i32> = Grid::try_from(vec![])?;
        assert_eq!(empty_grid.height(), 0);
        assert_eq!(empty_grid.width(), 0);

        Ok(())
    }

    #[test]
    fn test_grid_from_vec() -> Result<()> {
        let data = vec![vec![1, 2], vec![3, 4]];
        let grid: Grid<i32> = data.try_into()?;

        assert_eq!(grid[Point::new(0, 0)], 1);
        assert_eq!(grid[Point::new(1, 1)], 4);

        Ok(())
    }

    #[test]
    fn test_grid_get() -> Result<()> {
        let grid = Grid::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]])?;

        assert_eq!(grid.get(Point::new(0, 0)), Some(&1));
        assert_eq!(grid.get(Point::new(2, 1)), Some(&6));
        assert_eq!(grid.get(Point::new(3, 0)), None); // out of bounds x
        assert_eq!(grid.get(Point::new(0, 2)), None); // out of bounds y

        Ok(())
    }

    #[test]
    fn test_grid_get_mut() -> Result<()> {
        let mut grid = Grid::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]])?;

        if let Some(val) = grid.get_mut(Point::new(1, 1)) {
            *val = 99;
        }

        assert_eq!(grid[Point::new(1, 1)], 99);
        assert_eq!(grid.get_mut(Point::new(5, 5)), None); // out of bounds

        Ok(())
    }

    #[test]
    fn test_grid_iterator() -> Result<()> {
        let grid = Grid::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]])?;

        let mut row_count = 0;
        let mut sum = 0;
        for row in &grid {
            row_count += 1;
            for &val in row {
                sum += val;
            }
        }

        assert_eq!(row_count, 2);
        assert_eq!(sum, 21); // 1+2+3+4+5+6

        Ok(())
    }

    #[test]
    fn test_grid_iterator_mut() -> Result<()> {
        let mut grid = Grid::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]])?;

        for row in &mut grid {
            for val in row {
                *val *= 2;
            }
        }

        assert_eq!(grid[Point::new(0, 0)], 2);
        assert_eq!(grid[Point::new(2, 1)], 12);

        Ok(())
    }
}
