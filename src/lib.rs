pub mod grid;

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

fn input() -> Result<Vec<String>> {
    read_lines(INPUT_PATH)
}

fn test_input() -> Result<Vec<String>> {
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
    fn test_greatest_common_divisor() {
        assert_eq!(6, greatest_common_divisor(48, 18));
    }
}
