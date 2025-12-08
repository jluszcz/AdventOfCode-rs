pub mod grid;
pub mod logging;
pub mod math;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use anyhow::{Result, anyhow};
use clap::{Arg, ArgAction, Command};
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

    logging::init_logger(log_level)?;

    match input {
        Input::Actual => self::input(),
        Input::Test => test_input(),
    }
}

pub fn init_test() -> Result<Vec<String>> {
    logging::init_test_logger()?;
    test_input()
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
