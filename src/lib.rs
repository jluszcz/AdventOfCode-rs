use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use anyhow::{Result, anyhow};
use clap::{Arg, ArgAction, Command};
use env_logger::Target;
use log::{LevelFilter, trace};

const INPUT_PATH: &str = "input/input";
const TEST_INPUT_PATH: &str = "input/example";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Test,
    Actual,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "test" => Ok(Self::Test),
            "actual" => Ok(Self::Actual),
            _ => Err(anyhow!("Invalid input type: {}", s)),
        }
    }
}

pub fn init() -> Result<Vec<String>> {
    let matches = Command::new("advent-of-code")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("increase log level from the default for the input type"),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .default_value("actual")
                .help(format!(
                    "input type, {:?} or {:?}",
                    Input::Test,
                    Input::Actual
                )),
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");
    let input = matches
        .get_one::<String>("input")
        .map(|s| Input::from_str(s))
        .unwrap()?;

    let log_level = match (input, verbose) {
        (Input::Actual, false) => LevelFilter::Info,
        (Input::Actual, true) => LevelFilter::Debug,
        (Input::Test, false) => LevelFilter::Debug,
        (Input::Test, true) => LevelFilter::Trace,
    };

    init_logger(log_level)?;

    match input {
        Input::Actual => self::input(),
        Input::Test => test_input(),
    }
}

pub fn init_test() -> Result<Vec<String>> {
    init_test_logger()?;
    test_input()
}

fn init_logger(level: LevelFilter) -> Result<()> {
    inner_init_logger(Some(level), false)
}

pub fn init_test_logger() -> Result<()> {
    inner_init_logger(Some(LevelFilter::Trace), true)
}

fn inner_init_logger(level: Option<LevelFilter>, is_test: bool) -> Result<()> {
    let _ = env_logger::builder()
        .target(Target::Stdout)
        .filter_level(level.unwrap_or(LevelFilter::Info))
        .is_test(is_test)
        .try_init();

    Ok(())
}

pub fn input() -> Result<Vec<String>> {
    read_lines(INPUT_PATH)
}

pub fn test_input() -> Result<Vec<String>> {
    read_lines(TEST_INPUT_PATH)
}

fn read_lines(path: &'static str) -> Result<Vec<String>> {
    let lines: Vec<_> = BufReader::new(File::open(Path::new(path))?)
        .lines()
        .map_while(Result::ok)
        .inspect(|l| trace!("{}", l))
        .collect();

    if !lines.is_empty() {
        Ok(lines)
    } else {
        Err(anyhow!("No input: {}", path))
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

#[derive(Debug)]
pub struct MinMax {
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl FromIterator<usize> for MinMax {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut min = None;
        let mut max = None;

        for i in iter {
            min = match min {
                None => Some(i),
                Some(m) => Some(usize::min(m, i)),
            };
            max = match max {
                None => Some(i),
                Some(m) => Some(usize::max(m, i)),
            };
        }

        MinMax { min, max }
    }
}

pub fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b > a {
        greatest_common_divisor(b, a)
    } else if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

pub fn least_common_multiple(a: usize, b: usize) -> usize {
    (a * b) / greatest_common_divisor(a, b)
}

#[cfg(test)]
mod test {
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

    #[test]
    fn test_greatest_common_divisor() {
        assert_eq!(6, greatest_common_divisor(48, 18));
    }
}
